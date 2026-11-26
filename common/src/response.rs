//! TODO: docs.

use core::marker::PhantomData;

use http::StatusCode;
use http::response::Parts;

use crate::body::{self, Body, Json, WithMessage};

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

/// A type whose values are each one of the responses in `Owned`.
pub trait Alternatives: Sized {
    /// The responses, as [`Either`]s nested to the right in the order of the
    /// type's variants.
    type Owned;

    /// The `Owned` responses, borrowing from a value of the type.
    type Ref<'a>
    where
        Self: 'a;

    /// Returns the response `self` is.
    fn as_alternative(&self) -> Self::Ref<'_>;

    /// Returns the value of the type that is `alternative`.
    fn from_alternative(alternative: Self::Owned) -> Self;
}

/// TODO: docs.
pub trait Statuses {
    const STATUSES: &'static [StatusCode];

    fn status(&self) -> StatusCode;

    fn visit<B: Body, V: ResponseVisitor>(visitor: &mut V);
}

/// A type whose variants have doc comments.
#[doc(hidden)]
pub trait Descriptions {
    /// The doc comment of each of the type's variants.
    const DESCRIPTIONS: &'static [Option<&'static str>];
}

/// A response decoded from its parts and body.
pub trait Decode: Sized {
    /// Decodes the response with parts `parts` and body `body`.
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError>;
}

/// A response encoded into its status code and body.
pub trait Encode {
    /// Hands the response's status code and body to `encoder`.
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output;
}

/// Something done to the status code and body of a response.
pub trait Encoder {
    type Output;

    /// Encodes the response with status code `status` and body `body`.
    fn encode<B: body::Encode>(
        self,
        status: StatusCode,
        body: &B,
    ) -> Self::Output;
}

/// TODO: docs.
pub trait Response {
    /// Visits the status code and the body type of each of the type's
    /// responses.
    fn visit<V: ResponseVisitor>(visitor: &mut V);
}

/// A type whose values all respond with the same status code.
pub trait Status {
    /// The status code of the type's responses.
    const STATUS: StatusCode;
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

/// The ways a response fails to decode.
#[derive(Debug, derive_more::Display, cauchy::Error)]
pub enum DecodeError {
    /// The response's status code isn't one of the type's.
    #[display("the response's status code {_0} is unexpected")]
    Status(StatusCode),

    /// The response's `Content-Type` isn't the body's media type.
    #[display("the response's Content-Type isn't {expected}")]
    MediaType {
        /// The body's media type.
        expected: &'static str,
    },

    /// The response's body bytes don't decode.
    #[display("the response's body is invalid: {_0}")]
    Body(Box<dyn core::error::Error + Send + Sync>),
}

/// The response `A`, borrowed.
#[doc(hidden)]
pub struct Borrowed<'a, A>(pub &'a A);

/// The response `A` of the `I`th variant of `E`, described by that variant's
/// doc comment.
#[doc(hidden)]
pub struct Described<E, const I: usize, A>(pub A, pub PhantomData<E>);

/// The response `L`, or the response `R`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// A [`ResponseVisitor`] giving `description` to the responses visited
/// without one.
struct WithDescription<'a, V> {
    visitor: &'a mut V,
    description: &'static str,
}

/// A response with status code `S` and body `B`.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct WithStatus<const S: u16, B = ()>(pub B);

/// TODO: docs.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct WithStatusOf<B>(pub B);

/// Decodes `bytes` as the body `B` of the response with parts `parts`.
#[inline]
fn decode_body<B: body::Decode>(
    parts: &Parts,
    bytes: &[u8],
) -> Result<B, DecodeError> {
    if let Some(expected) = B::MEDIA_TYPE
        && !body::has_media_type(&parts.headers, expected)
    {
        return Err(DecodeError::MediaType { expected });
    }

    B::decode(bytes).map_err(|error| DecodeError::Body(Box::new(error)))
}

impl DecodeError {
    /// Returns `other` if it's more specific than `self`, or `self` if it
    /// isn't. Body errors are the most specific, and status code errors the
    /// least.
    #[inline]
    #[must_use]
    fn or(self, other: Self) -> Self {
        match (&self, &other) {
            (Self::Status(_), _) | (Self::MediaType { .. }, Self::Body(_)) => {
                other
            },
            _ => self,
        }
    }
}

impl<T> Decode for T
where
    T: Alternatives<Owned: Decode>,
{
    #[inline]
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError> {
        T::Owned::decode(parts, body).map(T::from_alternative)
    }
}

impl<E, const I: usize, A: Decode> Decode for Described<E, I, A> {
    #[inline]
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError> {
        A::decode(parts, body).map(|response| Self(response, PhantomData))
    }
}

/// The response `L`, or the response `R` if the response isn't an `L`.
impl<L: Decode, R: Decode> Decode for Either<L, R> {
    #[inline]
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError> {
        L::decode(parts, body).map(Self::Left).or_else(|error| {
            R::decode(parts, body)
                .map(Self::Right)
                .map_err(|other| error.or(other))
        })
    }
}

impl Decode for StatusCode {
    #[inline]
    fn decode(parts: &Parts, _: &[u8]) -> Result<Self, DecodeError> {
        Ok(parts.status)
    }
}

impl<const S: u16, B: body::Decode> Decode for WithStatus<S, B> {
    #[inline]
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError> {
        if parts.status != Self::STATUS {
            return Err(DecodeError::Status(parts.status));
        }

        decode_body(parts, body).map(Self)
    }
}

impl<B: body::Decode + Statuses> Decode for WithStatusOf<B> {
    #[inline]
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError> {
        if !B::STATUSES.contains(&parts.status) {
            return Err(DecodeError::Status(parts.status));
        }

        let body = decode_body::<B>(parts, body)?;

        if body.status() != parts.status {
            return Err(DecodeError::Status(parts.status));
        }

        Ok(Self(body))
    }
}

impl<T> Encode for T
where
    T: Alternatives,
    for<'a> T::Ref<'a>: Encode,
{
    #[inline]
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output {
        self.as_alternative().encode(encoder)
    }
}

impl<A: Encode> Encode for Borrowed<'_, A> {
    #[inline]
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output {
        self.0.encode(encoder)
    }
}

impl<L: Encode, R: Encode> Encode for Either<L, R> {
    #[inline]
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output {
        match self {
            Self::Left(response) => response.encode(encoder),
            Self::Right(response) => response.encode(encoder),
        }
    }
}

impl Encode for StatusCode {
    #[inline]
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output {
        encoder.encode(*self, &())
    }
}

impl<const S: u16, B: body::Encode> Encode for WithStatus<S, B> {
    #[inline]
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output {
        encoder.encode(Self::STATUS, &self.0)
    }
}

impl<B: body::Encode + Statuses> Encode for WithStatusOf<B> {
    #[inline]
    fn encode<E: Encoder>(&self, encoder: E) -> E::Output {
        encoder.encode(self.0.status(), &self.0)
    }
}

/// A successful response `T`, or an error response `E` if the response isn't a
/// `T`.
impl<T: Decode, E: Decode> Decode for Result<T, E> {
    #[inline]
    fn decode(parts: &Parts, body: &[u8]) -> Result<Self, DecodeError> {
        Either::<T, E>::decode(parts, body).map(|response| match response {
            Either::Left(response) => Ok(response),
            Either::Right(response) => Err(response),
        })
    }
}

impl<T> Response for T
where
    T: Alternatives<Owned: Response>,
{
    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        T::Owned::visit(visitor);
    }
}

impl<E, const I: usize, A> Response for Described<E, I, A>
where
    E: Descriptions,
    A: Response,
{
    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        match E::DESCRIPTIONS[I] {
            Some(description) => {
                A::visit(&mut WithDescription { visitor, description });
            },
            None => A::visit(visitor),
        }
    }
}

impl<L: Response, R: Response> Response for Either<L, R> {
    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        L::visit(visitor);
        R::visit(visitor);
    }
}

impl Response for StatusCode {
    #[inline]
    fn visit<V: ResponseVisitor>(_: &mut V) {}
}

impl<const S: u16, B: Body> Response for WithStatus<S, B> {
    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        visitor.visit::<B>(Self::STATUS, None);
    }
}

impl<B: Body + Statuses> Response for WithStatusOf<B> {
    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        B::visit::<B, V>(visitor);
    }
}

impl<const S: u16, B> Status for WithStatus<S, B> {
    const STATUS: StatusCode = match StatusCode::from_u16(S) {
        Ok(status) => status,
        Err(_) => panic!("status codes are between 100 and 999"),
    };
}

impl<T: Statuses> Statuses for Json<T> {
    const STATUSES: &'static [StatusCode] = T::STATUSES;

    #[inline]
    fn status(&self) -> StatusCode {
        self.0.status()
    }

    #[inline]
    fn visit<B: Body, V: ResponseVisitor>(visitor: &mut V) {
        T::visit::<B, V>(visitor);
    }
}

impl<T: Statuses> Statuses for WithMessage<T> {
    const STATUSES: &'static [StatusCode] = T::STATUSES;

    #[inline]
    fn status(&self) -> StatusCode {
        self.0.status()
    }

    #[inline]
    fn visit<B: Body, V: ResponseVisitor>(visitor: &mut V) {
        T::visit::<B, V>(visitor);
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
