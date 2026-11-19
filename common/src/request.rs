use crate::Body;

/// TODO: docs.
pub trait Request {
    /// TODO: docs.
    type Query: serde::Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema;

    /// TODO: docs.
    type Body: Body;
}

/// A request with query parameters `Q` and body `B`.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct WithQuery<Q, B = ()> {
    pub query: Q,
    pub body: B,
}

impl<B: Body> Request for B {
    type Query = ();
    type Body = Self;
}

impl<Q, B> Request for WithQuery<Q, B>
where
    Q: serde::Serialize + serde::de::DeserializeOwned + schemars::JsonSchema,
    B: Body,
{
    type Query = Q;
    type Body = B;
}
