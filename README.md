# Bit Scribe[rs]

Horrendous variable-length integer decoding in Rust.

## Usage

```bash
git clone https://github.com/masroof-maindak/bit-scribers.git
cd bit-scribers
cargo build -r
./target/release/bit-scribers <fpath> <offset>
```

## TODOs

- [x] Basic, working implementation
- [ ] Convert to library
- [ ] Encoding support

## Caveats

- Only works for Little-Endian systems. I can not be bothered to (do the tiny amount of work (I think) required to) add support for Big-Endian.
- ~~Assumes that the number will fit within 64 bits. 128 bit support be damned.~~

## Takeaways

- `rustup doc` is phenomenal
