# aws-profile-vault ![Rust](https://github.com/craigjbass/aws-profile-vault/workflows/Rust/badge.svg?branch=master)

This is intended as a **drop in replacement**_ish_ (you'll need aws-vault too) for `aws-profile`.

#### Why?

`aws-profile` is a tool that allows you to run arbitrary commands within specific aws profiles. It stores credentials in plaintext.

`aws-vault` is a tool that does roughly the same thing. One key difference is that _it encrypts your credentials in an appropriate backend_. This feels like a good idea.

Some teams have scripts that depend on aws-profile, in order to invoke commands across a variety of AWS accounts and/or roles.

This enables all team members to use `aws-vault`, but still use the legacy scripts that use `aws-profile`.

## Migrating from aws-profile

Both tools use profiles that can be configured via `~/.aws/config`.

1. If you are using `~/.aws/credentials` for profile mapping, port these to `~/.aws/config`
2. Delete `~/.aws/credentials`
3. Install aws-vault, and add your credentials from `~/.aws/credentials`.
4. Ensure aws-profile is removed
5. Add your credentials to aws-vault as appropriate

## Using

1. Download this tool from github releases and symlink it as `aws-profile` on your $PATH.
2. Use it like `aws-profile`

## Compiling

1. Clone the repository
2. Ensure you have [a Rust compiler installed](https://www.rust-lang.org/tools/install)
3. `cargo build --release`

## Running the tests

The entire project is defined in `src/main.rs`, including the tests.

Run `cargo test`

## Implementation Differences

**MFA tokens must be entered every time.**

This could be implemented, but this is aws-vault's default behaviour.

**1hr max sessions**

If your abitrary command needs to run for longer than 1hr, then the token will expire.

In order to support this use case, we'd need some kind of environment variable to set `aws-vault`'s `--no-session` flag.
