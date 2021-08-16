# Webber

Webber installs local shortcuts for webpages on Ubuntu touch.

<a href="https://open-store.io/app/webber.timsueberkrueb"><img width="256" src="https://open-store.io/badges/en_US.svg" alt="OpenStore" /></a>

## Build using clickable

Install [clickable](https://clickable-ut.dev), the build tool for Ubuntu touch apps.

To build and start the application on your phone, run:

```console
clickable
```

To build and start the application on your desktop, run:

```console
clickable desktop
```

## Build manually

Install Qt `5.12` or a newer Qt `5.X` release.

Install Rust nightly version `nightly-2021-08-13`.

```console
rustup override set nightly-2021-08-13
cargo build
```

Rust nightly is required until the following nightly features stabilize:

* [`generic_associated_types`](https://github.com/rust-lang/rust/issues/44265)

## Credits

Thanks to Hendrik Süberkrüb for creating the application icon.

Thanks to everyone who has contributed code or translations!

## License

Licensed under the terms of the GNU General Public License version 3 or, at your option, any later version.
