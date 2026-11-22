use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;

/// A `Vec<T>` of at most `MAX` elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(transparent)]
pub struct BoundedVec<T, const MAX: usize>(Vec<T>);

/// The error returned when converting a `Vec` into a [`BoundedVec`] fails
/// because it has more than `MAX` elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display)]
#[display("expected at most {max} elements, got {len}")]
pub struct BoundedVecTooLong {
    /// The maximum number of elements.
    pub max: usize,
    /// The number of elements.
    pub len: usize,
}

impl<T, const MAX: usize> BoundedVec<T, MAX> {
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T, const MAX: usize> Default for BoundedVec<T, MAX> {
    #[inline]
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T, const MAX: usize> Deref for BoundedVec<T, MAX> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MAX> {
    type Error = BoundedVecTooLong;

    #[inline]
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() > MAX {
            return Err(BoundedVecTooLong { max: MAX, len: vec.len() });
        }
        Ok(Self(vec))
    }
}

impl<T, const MAX: usize> IntoIterator for BoundedVec<T, MAX> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de, T, const MAX: usize> serde::Deserialize<'de> for BoundedVec<T, MAX>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        struct Visitor<T, const MAX: usize>(PhantomData<T>);

        impl<'de, T, const MAX: usize> serde::de::Visitor<'de> for Visitor<T, MAX>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = BoundedVec<T, MAX>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "a sequence of at most {MAX} elements")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let len = seq.size_hint().unwrap_or(0).min(MAX);
                let mut elements = Vec::with_capacity(len);

                while let Some(element) = seq.next_element()? {
                    if elements.len() == MAX {
                        return Err(serde::de::Error::invalid_length(
                            MAX + 1,
                            &self,
                        ));
                    }
                    elements.push(element);
                }

                Ok(BoundedVec(elements))
            }
        }

        deserializer.deserialize_seq(Visitor(PhantomData))
    }
}

impl<T: schemars::JsonSchema, const MAX: usize> schemars::JsonSchema
    for BoundedVec<T, MAX>
{
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> Cow<'static, str> {
        alloc::format!("BoundedVec_{}_{MAX}", T::schema_name()).into()
    }

    fn json_schema(
        generator: &mut schemars::SchemaGenerator,
    ) -> schemars::Schema {
        schemars::json_schema!({
            "type": "array",
            "items": generator.subschema_for::<T>(),
            "maxItems": MAX,
        })
    }
}

impl core::error::Error for BoundedVecTooLong {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_takes_up_to_max_elements() {
        let vec = serde_json::from_str::<BoundedVec<u8, 2>>("[1, 2]").unwrap();
        assert_eq!(*vec, [1, 2]);
    }

    #[test]
    fn deserialize_stops_past_max_elements() {
        let err = serde_json::from_str::<BoundedVec<u8, 2>>("[1, 2, 3]")
            .unwrap_err()
            .to_string();
        assert!(err.contains("at most 2 elements"), "{err}");
    }
}
