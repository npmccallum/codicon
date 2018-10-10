//
// Copyright 2018 Red Hat, Inc.
//
// Author: Nathaniel McCallum <npmccallum@redhat.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Traits for encoding and decoding.
//!
//! This crate provides generic traits for encoding and decoding to a
//! `std::io::Read` or `std::io::Write` type, respectively.
//!
//! We often need to express that a type can be encoded or decoded. We
//! also need a way to express the type of the encoding or decoding as
//! well as parameters that may be used for that encoding or decoding.
//! This tiny crate solves this problem.
//!
//! # Examples
//!
//! Let's say we want `u8` to be able to be encoded in a (made up) format `Foo`
//! which simply writes the byte without modification. We can express this
//! encoding as follows:
//!
//! ```rust
//! use std::io::{Result, Write};
//! use std::slice;
//!
//! use codicon::Encoder;
//!
//! struct Foo;
//!
//! impl Encoder<Foo> for u8 {
//!     fn encode<W: Write>(&self, writer: &mut W, params: Foo) -> Result<()> {
//!         writer.write_all(slice::from_ref(self))?;
//!         Ok(())
//!     }
//! }
//!
//! let mut buf = [0u8; 1];
//! 7u8.encode(&mut buf.as_mut(), Foo).unwrap();
//! assert_eq!(buf[0], 7u8);
//! ```
//!
//! Note that we used a unit struct because the `Foo` encoding doesn't take any
//! options. But if you wanted to specify encoding options, you could just make
//! a type with parameters.
//!
//! Decoding works the same as encoding:
//!
//! ```rust
//! use std::io::{Result, Read};
//! use std::slice;
//!
//! use codicon::Decoder;
//!
//! struct Foo;
//!
//! impl Decoder<Foo> for u8 {
//!     fn decode<R: Read>(reader: &mut R, params: Foo) -> Result<Self> {
//!         let mut byte = 0u8;
//!         reader.read_exact(slice::from_mut(&mut byte))?;
//!         Ok(byte)
//!     }
//! }
//!
//! let buf = [7u8; 1];
//! assert_eq!(u8::decode(&mut buf.as_ref(), Foo).unwrap(), 7u8);
//! ```

use std::io::{Error, Read, Write};

/// Trait used to express encoding relationships.
pub trait Encoder<T, E=Error> {
    /// Encodes to the writer with the given parameters.
    fn encode<W: Write>(&self, writer: &mut W, params: T) -> Result<(), E>;
}

/// Trait used to express decoding relationships.
pub trait Decoder<T, E=Error>: Sized {
    /// Decodes from the reader with the given parameters.
    fn decode<R: Read>(reader: &mut R, params: T) -> Result<Self, E>;
}
