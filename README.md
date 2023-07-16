# YaDI

[![Crates.io][crates-badge]][crates-url]
![MIT OR Apache-2.0 licensed][license-badge]

[crates-badge]: https://img.shields.io/crates/v/yadi.svg
[crates-url]: https://crates.io/crates/yadi
[license-badge]: https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg

Yet another dependency injection framework for writing applications with
the Rust programming language. It is:

* **Reliable**
* **Fast**
* **Easy-to-use**

# 🚨 Not implemented! 🚨

`yadi` has to be considered as a **DRAFT** project. Its API is unstable, most features are not implemented.
Please **DO NOT** use it in your projects until v1.0.0 release.

Stay tuned!

## Overview

`yadi` is a dependency injection framework for writing applications with
the Rust programming language. At a high level, it provides a few major components:

* A read-only container a.k.a [ServiceLocator].
* A builder for this container a.k.a [ServiceLocatorBuilder].
* Some helper structs, enums and types ([Tag], [Error], [Result], etc.)

These components are used to compose the framework for building a loose coupled and high cohesive application.

[ServiceLocator]: https://docs.rs/yadi/latest/yadi/struct.ServiceLocator.html
[ServiceLocatorBuilder]: https://docs.rs/yadi/latest/yadi/struct.ServiceLocatorBuilder.html
[Tag]: https://docs.rs/yadi/latest/yadi/struct.Tag.html
[Error]: https://docs.rs/yadi/latest/yadi/enum.Error.html
[Result]: https://docs.rs/yadi/latest/yadi/type.Result.html

## Example

A basic application with `yadi`

```rust,no_run
```

More examples can be found [here][examples]. For a larger "real world" example, see the
[flc] repository.

[examples]: https://github.com/FilaCo/yadi/tree/master/examples
[flc]: https://github.com/FilaCo/flc

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
