# tldr-rs

[![Build status](https://img.shields.io/travis/dbrgn/tldr-rs/master.svg)](https://travis-ci.org/dbrgn/tldr-rs)

An implementation of [tldr](https://github.com/tldr-pages/tldr) in Rust.

Project goals:

- [x] Don't require a network connection for anything besides updating the cache
- [x] Render tldr pages to ANSI terminals
- [x] Download and cache pages, compatible to the [NodeJS client][tldr-node-client]
- [ ] Command line interface similar or equivalent to the [NodeJS client][tldr-node-client]
- [x] Be fast

## Building

Debug build with logging enabled:

    $ cargo build --features logging

Release build without logging:

    $ cargo build --release

To enable the log output, set the `RUST_LOG` env variable:

    $ export RUST_LOG=tldr=debug

## License

MIT


[tldr-node-client]: https://github.com/tldr-pages/tldr-node-client
