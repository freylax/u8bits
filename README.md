# u8bits
rust macro to generate setters and getters for bits / bit ranges of u8 arrays

[![Build Status](https://travis-ci.org/freylax/u8bits.svg?branch=master)](https://travis-ci.org/freylax/u8bits)

This macro invocation:
```rust
struct Bytes( [u8;2]);
impl Bytes {
	u8bits! {
		/// foo is bit 4 of byte 0
		foo: rw 0,4;
		/// bar are bits 0..3 of byte 1
		u8, bar: rw 1,0,3;
	}
}
```
will generate get_ and set_ for foo and bar.
