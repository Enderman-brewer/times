# times

A time program, but something's off.

## Installation

### From source

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/your-username/times.git
cd times
cargo build --release
```

The binary will be at `target/release/times`.

### Arch Linux

Install from the package:

```bash
makepkg -si
```

## Usage

```
Usage: times [options]
Options:
  -u    Show Unix time (32-bit)
  -U    Show Unix time (64-bit)
  -S    Show UTC time
  -L    Show Local time
  -E    Show  time
  -P    Show pager time
  -F    The time, but forever
  -Y    Time until Y2K38
  -B    Web browser time
  -s    Attempt to use shaders
  -h, --help    Show help message
```

### Examples

```bash
times -L              # Local time
times -u              # Unix timestamp (32-bit)
times -S              # UTC time
times -Y              # Countdown to Y2K38
times -B              # Open current time in your browser
```

## Exit codes

| Code | Meaning |
|------|---------|
| 0    | Success |
| 1    | Unknown flag |
| 2    | No flags were passed |

## License

Do whatever you want with this.
