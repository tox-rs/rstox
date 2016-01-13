rstox
====

**rstox** is a Rust wrapper for [toxcore].
[![Build Status](https://travis-ci.org/suhr/rstox.svg)](https://travis-ci.org/suhr/rstox)

You need to have `toxcore` installed as dependency to use `rstox`. Either follow [install instructions](https://github.com/irungentoo/toxcore/blob/master/INSTALL.md), or, if you're running Gentoo, add Tox overlay and install `toxcore`:
```
# layman -f && layman -a tox-overlay && emerge net-libs/tox
```

To use `rstox` in your project, add to your `Cargo.toml`:
```
[dependencies.rstox]
git = "https://github.com/suhr/rstox.git"
```
and make something - [example](/examples/test.rs)

Toxcore [API documentation](https://github.com/irungentoo/toxcore/blob/master/toxcore/tox.h)


**rstox** is licensed under [GPLv3+](LICENSE)


[toxcore]:https://github.com/irungentoo/toxcore

## Contribuion

I do not develop rstox since I'm lazy and [tox development is not public](https://antox.me/my-time-at-tox.html). But pull requests are still welcome.

If you find an issue, consider fixing it yourself.
