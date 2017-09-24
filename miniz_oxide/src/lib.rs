//! A pure rust replacement for the [miniz](https://github.com/richgel999/miniz)
//! DEFLATE/zlib encoder/decoder.
//! The plan for this crate is to be used as a back-end for the
//! [flate2](https://github.com/alexcrichton/flate2-rs) crate and eventually remove the
//! need to depend on a C library.
//!
//! # Usage
//! ## Simple compression/decompression:
//! ``` rust
//!
//! use miniz_oxide::inflate::decompress_to_vec;
//! use miniz_oxide::deflate::compress_to_vec;
//!
//! fn roundtrip(data: &[u8]) {
//!     let compressed = compress_to_vec(data, 6);
//!     let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
//! #   let _ = decompressed;
//! }
//!
//! ```

extern crate adler32;
extern crate libc;

pub mod inflate;
pub mod deflate;
pub mod lib_oxide;
mod shared;
