# UDP Fanout

Mirrors UDP packets to multiple targets.

This can be useful for sending [StatsD](https://github.com/statsd/statsd) metrics to multiple metrics backends.

## Usage

See `udp-fanout --help` for usage.

For example: `udp-fanout -b 0.0.0.0:8125 -t 127.0.0.1:8126 -t 127.0.0.1:8127`, will bind to port `8125`, and mirror any UDP packets recieved to ports `8126` and `8127`.

## Building

- Install rust https://rustup.rs/

Compile:

    cargo build --release

You may wish to cross compile, e.g. compiling for Windows from a Linux host:

    rustup target add x86_64-pc-windows-gnu
    cargo build --target x86_64-pc-windows-gnu --release
