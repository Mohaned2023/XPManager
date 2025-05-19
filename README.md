## XPManager:
It's good to have a CLI tool that manages your passwords and lets you control them and quickly create new passwords of various sizes to suit your needs. This is where XPManager comes in to help you manage passwords, and also allows you to work with files/folders and secure them with the **Fernet** encryption.

## v2.2.0, What's New?
- Fix: password manager log registration logic.
- New: generate password with no symbols, see [generate](https://xpmanager.github.io/docs/usage/password-manager#generate).
- New: add custom set to password generate sample, see [generate](https://xpmanager.github.io/docs/usage/password-manager#generate).
- New: generate password from a custom sample, see [generate](https://xpmanager.github.io/docs/usage/password-manager#generate).

## Documentation:
See [XPManager documentation](https://xpmanager.github.io/docs/intro)

## Installation Instructions:
See [XPManager installation instructions](https://xpmanager.github.io/docs/installation)

## Install with Crates.io:
```sh
$ cargo install XPManager
$ xpm --version
```

## Cargo:
- Clone the repo:
```sh
$ git clone https://github.com/Mohaned2023/XPManager.git
$ cd XPManager
```
- Run
```sh
$ cargo run -- --version
```
- Test
```sh
$ cargo test
```
- Build with Release
```sh
$ cargo build --release
```
- Build deb package
```sh
$ cargo install cargo-deb
$ cargo deb
```

## Usage:
See [XPManager usage guide](https://xpmanager.github.io/docs/usage)

## Exit Codes:
See [XPManager exit codes](https://xpmanager.github.io/docs/errors)

---
> By [Mohaned Sherhan (Mr.x)](https://github.com/Mohaned2023)