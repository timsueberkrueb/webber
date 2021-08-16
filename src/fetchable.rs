use url::Url;

use crate::resolvable::*;

pub trait Fetchable {
    type Out<R: ResolveType>;
    type Error;

    fn fetch(url: &Url)
        -> Result<Self::Out<Unresolved>, Self::Error>;
}
