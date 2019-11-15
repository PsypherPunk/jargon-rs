# `jargon`

Originally downloaded from the
[Internet Archive](https://web.archive.org/web/20130827121341/http://cosman246.com/jargon.html).

Dump the text content of that page with:

```bash
lynx \
    --dump \
    --width 120 \
    --nonumbers \
    ${INTERNET_ARCHIVE_URL} > jargon.txt
```

Building the project with Cargo:

```bash
cargo build --release
```

This will update `src/lib.rs` in-place.
