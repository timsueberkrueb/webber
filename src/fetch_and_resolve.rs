use std::result::Result as StdResult;

use url::Url;

pub use crate::fetchable::*;
pub use crate::resolvable::*;

type Result<T> = StdResult<
    <<T as Fetchable>::Out<Unresolved> as Resolvable>::Out<Resolved>,
    <T as Fetchable>::Error,
>;

pub trait FetchAndResolve: Fetchable
where
    Self::Out<Unresolved>: Resolvable,
{
    fn fetch_and_resolve(url: &Url) -> Result<Self> {
        Self::fetch(url).map(|out| out.resolve(url))
    }
}

impl<T> FetchAndResolve for T
where
    T: Fetchable,
    <Self as Fetchable>::Out<Unresolved>: Resolvable,
{
}
