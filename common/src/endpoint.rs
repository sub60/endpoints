use core::fmt;

use http::{Method, StatusCode};

use crate::{Body, PathParams, Request, Response};

/// An HTTP endpoint.
pub trait Endpoint {
    /// The method of the endpoint's requests.
    const METHOD: Method;

    /// The template of the endpoint's path.
    const PATH: &'static str = <Self::Path as PathParams>::PATH;

    /// The parameters in the endpoint's path.
    type Path: PathParams;

    /// TODO: docs.
    type Request: Request;

    /// TODO: docs.
    type Response: Response;

    /// TODO: docs.
    type Error: fmt::Display + Into<StatusCode> + Body;
}

/// Something done to each of a domain's endpoints.
pub trait EndpointVisitor {
    /// Visits the endpoint.
    fn visit<E: Endpoint>(&mut self);
}
