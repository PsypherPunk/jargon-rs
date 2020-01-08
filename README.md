# `jargon`

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

Build using `cargo`:

```bash
cargo build --release
```

This will update `src/lib.rs` in-place.

Personally, I've created a symbolic link to the resulting binary and
added `jargon` to my `.bashrc` file:

```bash
ln -s $(pwd)/target/release/jargon ${HOME}/bin/jargon
```
