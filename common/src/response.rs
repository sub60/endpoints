//! TODO: docs.

use http::StatusCode;
pub use sub60_endpoints_macros::Response;

use crate::body::Body;

/// A `200 OK` response.
pub type Ok<B = ()> = WithStatus<200, B>;

/// A `201 Created` response.
pub type Created<B = ()> = WithStatus<201, B>;

/// A `202 Accepted` response.
pub type Accepted<B = ()> = WithStatus<202, B>;

/// A `204 No Content` response.
pub type NoContent = WithStatus<204>;

/// A `404 Not Found` response.
pub type NotFound<B = ()> = WithStatus<404, B>;

/// TODO: docs.
pub trait Response {
    /// Returns the response's status code.
    fn status(&self) -> StatusCode;

    /// Visits the status code and the body type of each of the type's
    /// responses.
    fn visit<V: ResponseVisitor>(visitor: &mut V);

    /// Visits the type held by each of the enum's variants.
    #[doc(hidden)]
    #[inline]
    fn visit_variants<V: VariantVisitor>(_visitor: &mut V) {}
}

/// TODO: docs.
pub trait ResponseVisitor {
    /// Visits a response with status code `status`, body `B` and an optional
    /// `description`.
    fn visit<B: Body>(
        &mut self,
        status: StatusCode,
        description: Option<&'static str>,
    );
}

/// Something done to the type held by each of an enum's variants.
#[doc(hidden)]
pub trait VariantVisitor {
    /// Visits `T`, the type held by the enum's `I`th variant.
    fn visit<T: Response, const I: usize>(&mut self);
}

/// A [`ResponseVisitor`] giving `description` to the responses visited
/// without one.
#[doc(hidden)]
pub struct WithDescription<'a, V> {
    pub visitor: &'a mut V,
    pub description: &'static str,
}

/// A response with status code `S` and body `B`.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct WithStatus<const S: u16, B = ()>(pub B);

impl<const S: u16, B> WithStatus<S, B> {
    /// The response's status code.
    pub const STATUS: StatusCode = match StatusCode::from_u16(S) {
        Ok(status) => status,
        Err(_) => panic!("status codes are between 100 and 999"),
    };
}

impl Response for StatusCode {
    #[inline]
    fn status(&self) -> StatusCode {
        *self
    }

    #[inline]
    fn visit<V: ResponseVisitor>(_: &mut V) {}
}

impl<const S: u16, B: Body> Response for WithStatus<S, B> {
    #[inline]
    fn status(&self) -> StatusCode {
        Self::STATUS
    }

    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        visitor.visit::<B>(Self::STATUS, None);
    }
}

impl<V: ResponseVisitor> ResponseVisitor for WithDescription<'_, V> {
    #[inline]
    fn visit<B: Body>(
        &mut self,
        status: StatusCode,
        description: Option<&'static str>,
    ) {
        self.visitor.visit::<B>(status, description.or(Some(self.description)));
    }
}
