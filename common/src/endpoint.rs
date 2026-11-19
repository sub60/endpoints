use core::fmt;

use http::{Method, StatusCode};

use crate::PathParams;

/// An HTTP endpoint.
pub trait Endpoint {
    /// The method of the endpoint's requests.
    const METHOD: Method;

    /// The template of the endpoint's path.
    const PATH: &'static str = <Self::Path as PathParams>::PATH;

    /// The ways the endpoint's requests can fail.
    type Error: fmt::Display + Into<StatusCode>;

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
        Error: serde::Serialize + schemars::JsonSchema,
        Request: schemars::JsonSchema + 'static,
        Response: schemars::JsonSchema,
    >
{
}

/// Something done to each of a domain's endpoints.
pub trait EndpointVisitor {
    /// Visits the endpoint.
    fn visit<E: Endpoint>(&mut self);
}

/// Same as [`EndpointVisitor`], but for [`JsonEndpoint`]s.
pub trait JsonEndpointVisitor {
    /// Visits the endpoint.
    fn visit<E: JsonEndpoint>(&mut self);
}

impl<E> JsonEndpoint for E where
    E: Endpoint<
            Error: serde::Serialize + schemars::JsonSchema,
            Request: schemars::JsonSchema + 'static,
            Response: schemars::JsonSchema,
        >
{
}

impl<V: EndpointVisitor> JsonEndpointVisitor for V {
    #[inline]
    fn visit<E: JsonEndpoint>(&mut self) {
        EndpointVisitor::visit::<E>(self);
    }
}
