[![Build Status](https://travis-ci.org/psilocybin/codicon.svg?branch=master)](https://travis-ci.org/psilocybin/codicon)
![Rust Version 1.28+](https://img.shields.io/badge/rustc-v1.28%2B-blue.svg)
[![Crate](https://img.shields.io/crates/v/codicon.svg)](https://crates.io/crates/codicon)
[![Docs](https://docs.rs/codicon/badge.svg)](https://docs.rs/codicon)

Codicon is a crate containing two simple traits (`Encoder` and `Decoder`) that
express relationships between a native Rust type and its possible encodings.

We often need to express that a type can be encoded or decoded. We also need a
way to express the type of the encoding or decoding as well as parameters that
may be used for that encoding or decoding. This tiny crate solves this problem.

# Install

Run this command:

    $ cargo add codicon

# Examples

Let's say we want `u8` to be able to be encoded in a (made up) format `Foo`
which simply writes the byte without modification. We can express this
encoding as follows:

```rust
use std::io::{Result, Write};
use std::slice;

use codicon::Encoder;

struct Foo;

impl Encoder<Foo> for u8 {
    fn encode<W: Write>(&self, writer: &mut W, params: Foo) -> Result<()> {
        writer.write_all(slice::from_ref(self))?;
        Ok(())
    }
}

let mut buf = [0u8; 1];
7u8.encode(&mut buf.as_mut(), Foo).unwrap();
assert_eq!(buf[0], 7u8);
```

Note that we used a unit struct because the `Foo` encoding doesn't take any
options. But if you wanted to specify encoding options, you could just make
a type with parameters.

Decoding works the same as encoding:

```rust
use std::io::{Result, Read};
use std::slice;

use codicon::Decoder;

struct Foo;

impl Decoder<Foo> for u8 {
    fn decode<R: Read>(reader: &mut R, params: Foo) -> Result<Self> {
        let mut byte = 0u8;
        reader.read_exact(slice::from_mut(&mut byte))?;
        Ok(byte)
    }
}

let buf = [7u8; 1];
assert_eq!(u8::decode(&mut buf.as_ref(), Foo).unwrap(), 7u8);
```
