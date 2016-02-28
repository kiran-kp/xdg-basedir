xdg-basedir
====

[![Build Status](https://travis-ci.org/kiran-kp/xdg-basedir.svg)](https://travis-ci.org/kiran-kp/xdg-basedir) [![](http://meritbadge.herokuapp.com/xdg-basedir)](https://crates.io/crates/xdg-basedir)

[Documentation](http://kiran-kp.github.io/xdg-basedir/xdg_basedir/index.html)

xdg-basedir is a utility library to make conforming to the
[XDG basedir specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.

#Example
```rust
#![cfg(unix)]
extern crate xdg_basedir;

#![cfg(unix)]
use xdg_basedir::*;
use std::path::PathBuf;
...
let data_home: PathBuf = try!(get_data_home());
...
```

**Unstable features:**
- Test runtime directory: A function to check if a directory satisfies the XDG spec's requirements of a runtime directory.

The default build of xdg-basedir does not use any unstable libstd features. To enable them, you'll need to use the nightly build of rustc and build xdg-basedir with the 'unstable' feature toggle.

```toml
[dependencies.xdg-basedir]
version = "0.2.2"
features = ["unstable"]
```

Alternate implementation and some initial source borrowed from [rust-xdg](https://github.com/o11c/rust-xdg).
The APIs provided by ```rust-xdg``` and ```xdg-basedir``` are different.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
