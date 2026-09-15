// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod execution_atom;
pub mod pipeline_cx;
pub mod request_cx;
pub mod response_cx;

pub use execution_atom::*;
pub use pipeline_cx::*;
pub use request_cx::*;
pub use response_cx::*;

use crate::*;

use databases::*;

pub type PipelineResult = Result<(PipelineCx, PipelineAction), RequestError>;

/// Generic executor trait - this is the main trait that endpoint executors will implement.
#[typetag::serde]
#[async_trait]
pub trait AnyHttpExecutor: AnyExt {
    /// Executes a query using the given executor and database connection.
    async fn execute(&self, cx: PipelineCx, db_conns: DbConns) -> PipelineResult;
}
