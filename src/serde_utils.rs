use serde::de::Deserializer;
use serde::Deserialize;
use serde_json::Value;

/// Deserialize an `Option<T>`, mapping any error to `None` that occurs while deserializing `T`
#[derive(Debug)]
struct IgnoreError<T>(Option<T>);

impl<T> IgnoreError<T> {
    fn into_inner(self) -> Option<T> {
        self.0
    }
}

impl<'de, T> Deserialize<'de> for IgnoreError<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ok_or_none(d).map(Self)
    }
}

/// A `Vec` which doesn't fail to deserialize if some items fail to deserialize
#[derive(Debug)]
pub struct LossyVec<T>(Vec<IgnoreError<T>>);

impl<T> From<Vec<Option<T>>> for LossyVec<T> {
    fn from(other: Vec<Option<T>>) -> Self {
        Self(other.into_iter().map(IgnoreError).collect())
    }
}

impl<T> LossyVec<T> {
    /// Iterate over the successfully deserialized items
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().filter_map(|x| x.0.as_ref())
    }

    /// Map over the successfully deserialized items
    pub fn purge_map<B, F>(self, f: F) -> Vec<B>
    where
        F: Fn(T) -> B,
    {
        self.0
            .into_iter()
            .filter_map(|x| x.into_inner().map(&f))
            .collect()
    }
}

impl<'de, T> Deserialize<'de> for LossyVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<IgnoreError<T>>::deserialize(d).map(LossyVec)
    }
}

pub fn ok_or_none<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    ok_or_default(d)
}

pub fn ok_or_default<'de, T, D>(d: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    match Option::<Value>::deserialize(d)? {
        None => Ok(T::default()),
        Some(value) => match T::deserialize(value) {
            Ok(t) => Ok(t),
            Err(_) => Ok(T::default()),
        },
    }
}
