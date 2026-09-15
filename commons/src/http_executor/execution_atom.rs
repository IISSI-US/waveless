// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use super::*;

/// Execution atom (node) for composing the tree-like structure.
#[serde_as]
#[derive(Clone, Constructor, Serialize, Deserialize, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct ExecutionAtom {
    /// Unique identifier for each executor.
    /// NOTE: if the id is not set the parent node won't be able to select this step if it has multiple children.
    #[serde(default, skip_serializing_if = "should_skip_option")]
    id: Option<ExecutionAtomId>,

    /// Establishes the execute handler.
    #[serde_as(as = "IfIsHumanReadable<_, JsonString>")] // Explore müsli to avoid this.
    executor: Arc<dyn AnyHttpExecutor>,

    /// All children of the current node.
    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    children: CheapVec<Arc<ExecutionAtom>>,
}

impl<T: AnyHttpExecutor> From<Arc<T>> for ExecutionAtom {
    fn from(value: Arc<T>) -> Self {
        ExecutionAtom::new(None, value, CheapVec::new_const())
    }
}

impl ExecutionAtom {
    /// Given an execution atom execute it and execute all it's children recursively.
    pub fn execute<'a>(
        &'a self,
        cx: PipelineCx,
        db_conns: DbConns,
    ) -> BoxFuture<'a, Result<PipelineCx, RequestError>> {
        let future: Pin<_> = Box::pin(async move {
            let ExecutionAtom {
                id,
                executor,
                children,
                ..
            } = self;

            // Run the current executor.
            let (PipelineCx { request, response }, action) =
                executor.execute(cx, db_conns.to_owned()).await?;

            match action {
                PipelineAction::Continue(child_id) if !children.is_empty() => {
                    // Get the children with the matching id.
                    let child = {
                        match &child_id {
                            Some(child_id) => children.iter().find(|child| {
                                child.id().to_owned().unwrap_or_default() == child_id
                            }),
                            None => children.iter().next(),
                        }
                    }
                    .wrap_err(format!(
                        "Execution atom `{}` doesn't have a child with the id `{}`",
                        id.to_owned().unwrap_or("no id".into()),
                        child_id.to_owned().unwrap_or_default()
                    ))?
                    .to_owned();

                    Ok(child.execute((request, response).into(), db_conns).await?)
                }
                _ => Ok((request, response).into()), // Discards any further execution and returns the current response.
            }
        });

        future as BoxFuture<_> // `rust-analyzer` complains here.
    }
}
