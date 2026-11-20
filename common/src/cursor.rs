use alloc::borrow::Cow;
use alloc::string::String;

/// An opaque position in a paginated list.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Cursor(String);

impl Cursor {
    /// Returns the cursor's string.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates a cursor from its string.
    #[inline]
    pub const fn new(cursor: String) -> Self {
        Self(cursor)
    }
}

impl schemars::JsonSchema for Cursor {
    fn schema_name() -> Cow<'static, str> {
        "Cursor".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "An opaque position in a paginated list.",
            "type": "string",
        })
    }
}
