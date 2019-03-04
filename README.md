# rstox

[![Build Status](https://travis-ci.org/tox-rs/rstox.svg)](https://travis-ci.org/tox-rs/rstox)

**rstox** is a Rust wrapper for [toxcore].

You need to have `toxcore` installed as dependency to use `rstox`. Follow the [install instructions](https://github.com/TokTok/c-toxcore/blob/master/INSTALL.md).

To use `rstox` in your project, add to your `Cargo.toml`:

```
[dependencies.rstox]
git = "https://github.com/tox-rs/rstox.git"
```
and make something - [example](/examples/test.rs)

Toxcore [API documentation](https://github.com/TokTok/c-toxcore/blob/master/toxcore/tox.h)

**rstox** is licensed under [GPLv3+](LICENSE)

[toxcore]:https://github.com/TokTok/c-toxcore
