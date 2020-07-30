# `jargon`

![audit-lint](https://github.com/PsypherPunk/jargon-rs/workflows/audit-lint/badge.svg)
![release](https://github.com/PsypherPunk/jargon-rs/workflows/release/badge.svg)

## About

Spawned from a
[bout of nostalgia and some free time](https://blog.psypherpunk.io/posts/jargon/).

The contents of `jargon.txt.gz` were originally derived from the
[Internet Archive](https://web.archive.org/web/20130827121341/http://cosman246.com/jargon.html):

```bash
lynx \
    --dump \
    --width 120 \
    --nonumbers \
    ${INTERNET_ARCHIVE_URL} | gzip --best -c > jargon.txt.gz
```

## Build

Build using `cargo`:

```bash
cargo build --release
```

This will update `src/lib.rs` in-place. The resulting changes can be
ignored with:

```bash
git update-index \
    --assume-unchanged \
    src/lib.rs
```

## Running

This can be run locally using `cargo`:

```bash
cargo run --release
```

Alternatively, having built it as above, the resulting binary located
in `target/release/jargon` can be executed directly:

```bash
./target/release/jargon
```

## Releases

[Releases](https://github.com/PsypherPunk/jargon-rs/releases) are
available as `deb` packages.
