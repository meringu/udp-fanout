# UDP Fanout

Mirrors UDP packets to multiple targets.

This can be useful for sending [StatsD](https://github.com/statsd/statsd) metrics to multiple metrics backends.

## Usage

For a given `config.toml` file:

```toml
bind_address = "0.0.0.0:8125"
targets = [
    "127.0.0.1:8126",
    "127.0.0.1:8127",
]
```

Run:

```
udp-fanout -c config.toml
```

This will `udp-fanout` bind to port `8125`, and mirror any UDP packets recieved to ports `8126` and `8127`.
