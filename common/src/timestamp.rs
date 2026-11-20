use alloc::borrow::Cow;
use core::time::Duration;

/// A point in time with second-level precision.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Returns the number of seconds between the UNIX epoch and the
    /// timestamp.
    #[inline]
    pub const fn as_secs(self) -> u64 {
        self.0
    }

    /// Creates the timestamp `secs` seconds after the UNIX epoch.
    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        Self(secs)
    }
}

/// The timestamp `duration` after the UNIX epoch, rounded down to the second.
impl From<Duration> for Timestamp {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self(duration.as_secs())
    }
}

/// The time between the UNIX epoch and the timestamp.
impl From<Timestamp> for Duration {
    #[inline]
    fn from(timestamp: Timestamp) -> Self {
        Self::from_secs(timestamp.0)
    }
}

impl serde::Serialize for Timestamp {
    #[inline]
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        u64::deserialize(deserializer).map(Self)
    }
}

impl schemars::JsonSchema for Timestamp {
    fn schema_name() -> Cow<'static, str> {
        "Timestamp".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "A point in time, as the number of seconds since the UNIX epoch.",
            "type": "integer",
            "minimum": 0,
        })
    }
}
