use url::Url;

use serde::de::DeserializeOwned;

use crate::serde_utils::*;

pub trait ResolveType {
    // FIXME: Workaround for likely Rust and/or serde `Deserialize` derive bug/limitation that shows
    // up in `pwa::parser::ShortcutItem<_>::icons`:
    //
    // error: implementation of `Deserialize` is not general enough
    // --> src/pwa/parser.rs:109:16
    //     |
    // 109 |     pub icons: Option<R::Array<Icon<R>>>,
    //     |                ^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Deserialize`
    //     |                                          is not general enough
    //     |
    //     = note: `<R as resolvable::ResolveType>::Url` must implement `Deserialize<'0>`,
    //             for some specific lifetime `'0`...
    type This: ResolveType;
    type Url: DeserializeOwned;
    type Array<T: DeserializeOwned>: DeserializeOwned;
}

#[derive(Debug, Default)]
pub struct Unresolved;

impl ResolveType for Unresolved {
    type This = Unresolved;
    type Url = String;
    type Array<T: DeserializeOwned> = LossyVec<T>;
}

#[derive(Debug, Default)]
pub struct Resolved;

impl ResolveType for Resolved {
    type This = Resolved;
    type Url = url::Url;
    type Array<T: DeserializeOwned> = Vec<T>;
}

pub trait Resolvable {
    type Out<R: ResolveType>;

    fn resolve(self, base_url: &Url) -> Self::Out<Resolved>;
}

pub trait ResolvableConst {
    type Out;

    fn resolve(self) -> Self::Out;
}

pub trait ResolvableComposite {
    type Composite<T>;
    type Resolvable: Resolvable;

    fn resolve(
        self,
        base_url: &Url,
    ) -> <Self as ResolvableComposite>::Composite<
        <<Self as ResolvableComposite>::Resolvable as Resolvable>::Out<Resolved>,
    >;
}

impl Resolvable for String {
    type Out<R: ResolveType> = Option<Url>;

    fn resolve(self, base_url: &Url) -> Option<Url> {
        base_url.join(&self).ok()
    }
}

impl<T: Resolvable> Resolvable for Option<T> {
    type Out<R: ResolveType> = Option<<T as Resolvable>::Out<R>>;

    fn resolve(self, base_url: &Url) -> Self::Out<Resolved> {
        self.map(|inner| inner.resolve(base_url))
    }
}

impl<T: ResolvableConst> ResolvableConst for Option<T> {
    type Out = Option<<T as ResolvableConst>::Out>;

    fn resolve(self) -> Self::Out {
        self.map(|inner| inner.resolve())
    }
}

impl<T> ResolvableConst for LossyVec<T> {
    type Out = Vec<T>;

    fn resolve(self) -> Vec<T> {
        self.purge_map(|x: T| x)
    }
}

impl<T: Resolvable> ResolvableComposite for LossyVec<T> {
    type Composite<S> = Vec<S>;
    type Resolvable = T;

    fn resolve(
        self,
        base_url: &Url,
    ) -> <Self as ResolvableComposite>::Composite<
        <<Self as ResolvableComposite>::Resolvable as Resolvable>::Out<Resolved>,
    > {
        self.purge_map(|x: T| x.resolve(base_url))
    }
}

impl<C: ResolvableComposite> Resolvable for C {
    type Out<R: ResolveType> = <Self as ResolvableComposite>::Composite<
        <<Self as ResolvableComposite>::Resolvable as Resolvable>::Out<R>,
    >;

    fn resolve(self, base_url: &Url) -> Self::Out<Resolved> {
        <Self as ResolvableComposite>::resolve(self, base_url)
    }
}
