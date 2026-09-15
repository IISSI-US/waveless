// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use super::*;

/// Contains both the current context of the pipeline, passing the request and the candidate response.
#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct PipelineCx {
    /// Incoming request from the client (which might have been modified
    /// by a pipeline's step).
    pub request: RequestCx,

    /// Outbound response (which might be modified by children execution steps).
    /// NOTE: if there is no response set, an empty 200 response will be sent back.
    pub response: Option<ResponseCx>,
}

impl From<(RequestCx, ResponseCx)> for PipelineCx {
    fn from(value: (RequestCx, ResponseCx)) -> Self {
        let (request, response) = value;
        PipelineCx {
            request,
            response: Some(response),
        }
    }
}

impl From<(RequestCx, Option<ResponseCx>)> for PipelineCx {
    fn from(value: (RequestCx, Option<ResponseCx>)) -> Self {
        let (request, response) = value;
        PipelineCx { request, response }
    }
}

impl From<PipelineCx> for (RequestCx, Option<ResponseCx>) {
    fn from(value: PipelineCx) -> Self {
        let PipelineCx {
            request, response, ..
        } = value;

        (request, response)
    }
}

impl PipelineCx {
    /// Creates a new pipeline context with the given request.
    pub fn new(request: RequestCx) -> Self {
        Self {
            request,
            response: None,
        }
    }
}

/// Determines whether to continue execution with the optionally given child
/// or abort the execution and return the current response.
#[derive(Default, Debug)]
pub enum PipelineAction {
    /// Continue to the next established executor.
    /// NOTE: if there is none it will finish.
    /// NOTE: if there are multiple children the first one will be executed.
    Continue(Option<ExecutionAtomId>),

    /// Finish execution flow and immediately return the current response.
    #[default]
    Finish,
}
