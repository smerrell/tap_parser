# Rust Tap Harness [![Build Status](https://travis-ci.org/smerrell/tap_parser.svg)](https://travis-ci.org/smerrell/tap_parser)

A [TAP](https://testanything.org/) Harness built in Rust. The goal of this
project is to build a TAP harness that adheres to the [TAP 13
specification](https://testanything.org/tap-version-13-specification.html) but
can also handle output from the real-world.

## Using the harness

```bash
$ <some command outputting TAP> | tap_parser
```

**Note:** This assumes you have the `tap_parser` executable somewhere in your
`PATH`.

### From Source

Currently this is the only way to use this as this is not yet published on
[Cargo](https://crates.io).

1. Clone the repo
1. `cargo build --release`
1. Add `<path to the repo>/target/release/` to your `PATH`

### From Cargo

*Not yet ready for Cargo*

## Documentation

For now, the source is the best place to look but there will be Rustdoc for this
going forward. This project is in quite an early state.

## Contributing

Things are still in a pretty early phase and the Rust is not very idiomatic but
contributions are welcome if you're interested in helping.

## License

This project is licensed under the [MIT](https://opensource.org/licenses/MIT)
license. See the LICENSE file for the full text of the license.
