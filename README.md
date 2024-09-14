# About

Like dotenv but with `.cargo/config.toml` files

```text
$ env |grep ^CARGO_ENV_RUN_TEST
$ cat .cargo/config.toml
[env]
CARGO_ENV_RUN_TEST = "IT WORKS"
$ cargo env-run env |grep ^CARGO_ENV_RUN_TEST
CARGO_ENV_RUN_TEST=IT WORKS
```

This crate/utility does not intend to replace [`dotenv`] or similar utilites, and was originally
designed to overcome [Helix Editor]'s inability to recognize a compile-time environment variable
being set in a `.cargo/config.toml` file instead of the current environment.
Using this utility, the fix for this scenario is to prefix the `hx src/main.rs` command with
`cargo env-run ` which loads environment variables from `.cargo/config.toml` files and then runs
the given command.

[`dotenv`]: https://crates.io/crates/dotenv
[Helix Editor]: https://helix-editor.com/

