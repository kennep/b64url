# b64url

This is a simple URL-safe Base64 encoder/decoder written in Rust.

![Build status](https://github.com/kennep/b64url/actions/workflows/ci.yml/badge.svg)

## Download 

Pre-compiled binaries for Linux, Windows and (Intel) macOS are available on the
[releases](https://github.com/kennep/b64url/releases/latest) page.

## Building

If you have Rust and Cargo installed, you can download the source and build it yourself
in the regular way:

```bash
$ cargo build
```

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

## Feedback

This is my first Rust program, so go easy on me :) That being said, feedback, PRs, etc. are most welcome.

