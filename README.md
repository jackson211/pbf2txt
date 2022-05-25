# `pbf2txt`

**Convert OSM's `pbf` file to `txt` format.**

Simple CLI tool to convert pbf file into lat and lon only txt

## Build

```bash
cargo build --release
```

## Example

```bash
./target/release/pbf2txt -i andorra-latest.osm.pbf
```

CLI attributes:

- `-i [INPUT_PATH]`: input file path
- `-o [OUTPUT_PATH]`: optional output file path
- `-s`: optional show statistics of lat and lon

# License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
