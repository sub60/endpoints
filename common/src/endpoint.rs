use http::Method;

use crate::{EndpointError, PathParams};

/// An HTTP endpoint.
pub trait Endpoint {
    /// The method of the endpoint's requests.
    const METHOD: Method;

    /// The template of the endpoint's path.
    const PATH: &'static str = <Self::Path as PathParams>::PATH;

    /// The parameters in the endpoint's path.
    type Path: PathParams;

    /// The body of the endpoint's requests.
    type Request;

    /// The endpoint's successful responses.
    type Response;
}

/// An [`Endpoint`] whose requests, successful responses and errors are JSON.
pub trait JsonEndpoint:
    Endpoint<
        Request: schemars::JsonSchema + 'static,
        Response: schemars::JsonSchema,
    >
{
    /// The ways the endpoint's requests can fail.
    type Error: EndpointError;
}

/// Something done to each of a domain's endpoints.
pub trait EndpointVisitor {
    /// Visits the endpoint.
    fn visit<E: JsonEndpoint>(&mut self);
}
