# b64url

This is a simple URL-safe Base64 encoder/decoder written in Rust.

## Usage

It is designed to be used as a filter and reads the data to be encoded or decoded from standard input.

To encode something:

```bash
$ echo "You fight like a dairy Farmer!" | b64url -e
WW91IGZpZ2h0IGxpa2UgYSBkYWlyeSBGYXJtZXIh
```

To decode something:

```bash
$ echo "SG93IGFwcHJvcHJpYXRlLiBZb3UgZmlnaHQgbGlrZSBhIGNvdyE" | b64url -d
How appropriate. You fight like a cow!
```

The default, if neither `-e` nor `-d` is given, is to encode the input.

## Building

```bash
$ cargo build
```

## Feedback

This is my first Rust program, so go easy on me :) That being said, feedback, PRs, etc. are most welcome.

