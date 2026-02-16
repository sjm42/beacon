# beacon

A lightweight Rust CLI utility that periodically outputs a message. Useful as a heartbeat signal, keep-alive, or simple periodic logger.

## Usage

```
beacon [OPTIONS]
```

### Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--message` | `-m` | Message to output | `ping` |
| `--interval` | `-i` | Seconds between outputs | `60` |
| `--timestamp` | `-t` | Prefix output with timestamps | off |
| `--verbose` | `-v` | Enable INFO-level logging | off |
| `--debug` | `-d` | Enable DEBUG-level logging | off |
| `--trace` | | Enable TRACE-level logging | off |

### Examples

```sh
# Output "ping" every 60 seconds
beacon

# Output "heartbeat" every 30 seconds with timestamps
beacon -t -m heartbeat -i 30

# Verbose mode shows startup info
beacon -v
```

## Building

```sh
cargo build --release
```

## License

MIT
