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

`toxcore` API documentation: https://libtoxcore.so


**rstox** is licensed under [LGPLv3](LICENSE)


[toxcore]:https://github.com/irungentoo/toxcore
