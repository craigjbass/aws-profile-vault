# aws-profile-vault ![Rust](https://github.com/craigjbass/aws-profile-vault/workflows/Rust/badge.svg?branch=master)

#### Why?

`aws-profile` is a tool that allows you to run arbitrary commands within specific aws profiles. It stores credentials in plaintext.

`aws-vault` is a tool that does the same thing. It encrypts your credentials in an appropriate backend.

Some teams have scripts that depend on aws-profile and/or aws-vault. 

This enables all team members to use aws-vault, but still execute their scripts that use aws-profile.

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

Run `cargo test`

## Implementation Differences

**MFA tokens must be entered every time.**

This could be implemented, but this is aws-vault's default behaviour.

**1hr max sessions**

If your abitrary command needs to run for longer than 1hr, then the token will expire.

In order to support this use case, we'd need some kind of environment variable to set `aws-vault`'s `--no-session` flag.
