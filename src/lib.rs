#![no_std]
#![deny(
    missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

//! A macro to generate bit field like getters and setters on structs which
//! represent an array of bytes.
//!
//!  See the documentation of the macro for how to use it.

pub use paste;

/// Set the given 'value' in byte 'b' in the range of the given bit positions
/// 'lsb' (least significant bit) and 'msb' (most significant bit) inclusive.
/// the other bits are not touched.
pub fn set_bit_range(b: &mut u8, lsb: u8, msb: u8, value: u8) {
    let mask: u8 = !0u8 << (7 - msb) >> (7 - msb + lsb) << lsb;
    *b &= !mask;
    *b |= (value << lsb) & mask;
}
/// Return the 'value' of byte 'b' in the range of the given bit positions
/// 'lsb' (least significant bit) and 'msb' (most significant bit) inclusive.
pub fn get_bit_range(b: &u8, lsb: u8, msb: u8) -> u8 {
    let mask: u8 = !0u8 << (7 - msb) >> (7 - msb + lsb) << lsb;
    (*b & mask) >> lsb
}
/// Set the bit at position 'pos' of byte 'b'.  
pub fn set_bit(b: &mut u8, pos: u8, value: bool) {
    let mask: u8 = !1u8 << pos;
    *b &= !mask; if value { *b |= mask; }
}
/// Get the bit at position 'pos' of byte 'b'.
pub fn get_bit(b: &u8, pos: u8) -> bool {
    let mask: u8 = !1u8 << pos;
    (*b & mask) == mask
}


/// This macro allows the generaion of bit field like getters
/// and setters for a New Type struct of type [u8;N].
/// There are bit wise accessors and bit range accessors.
/// The bitwise operations map to an boolean value whereas the
/// bit range operations use an provided type which has to implement
/// the .into() conversion trait. 
/// The bit wise field declaration is as follows:
///
/// * Optional attributes (`#[...]`), documentation comments (`///`) are attributes; 
/// * the identifier, which will be suffixed by 'set_' and / or 'get_'
/// * A colon
/// * an 'r' or 'w' or 'rw' flag which indicates which accessor are created
///   (read->get,write->set or both)
/// * the byte number followed by a comma
/// * the bit number
///
/// The bit range operations are as follows:
/// 
/// * Optional attributes (`#[...]`), documentation comments (`///`) are attributes; 
/// * An type, which has to provide the core conversion Into<u8> trait, followed by a comma
/// * the identifier, which will be suffixed by 'set_' and / or 'get_'
/// * A colon
/// * an 'r' or 'w' or 'rw' flag which indicates which accessor are created
///   (read->get,write->set or both)
/// * the byte number followed by a comma
/// * the lsb (least significant) bit number
/// * the msb (most significant) bit number inclusive
///
/// So a typical declaration might look like:  
/// ```rust
/// use u8bits::u8bits;
///
/// struct Bytes( [u8;2]);
/// impl Bytes {
///    u8bits! {
///        /// foo is bit 4 of byte 0
///        foo: rw 0,4;
///        /// bar are bits 0..3 of byte 1
///        u8, bar: rw 1,0,3;
///    }
/// }
/// 
/// ```
/// which will expand to:
/// ```rust
/// struct Bytes([u8; 2]);
///     impl Bytes {
///         #[inline]
///         #[doc = r" foo is bit 4 of byte 0"]
///         pub fn get_foo(&self) -> bool { ::u8bits::get_bit(&self.0[0], 4) }
///         #[inline]
///         #[doc = r" foo is bit 4 of byte 0"]
///         pub fn set_foo(&mut self, value: bool) {
///             ::u8bits::set_bit(&mut self.0[0], 4, value)
///         }
///         #[inline]
///         #[doc = r" bar are bits 0..3 of byte 1"]
///         pub fn get_bar(&self) -> u8 {
///             ::u8bits::get_bit_range(&self.0[1], 0, 3).into()
///         }
///         #[inline]
///         #[doc = r" bar are bits 0..3 of byte 1"]
///         pub fn set_bar(&mut self, value: u8) {
///             ::u8bits::set_bit_range(&mut self.0[1], 0, 3, value.into())
///         }
///     }
/// ```
///
/// For the use of bitfield like enums see for example the
/// [surjective-enum](https://crates.io/crates/surjective-enum)
/// crate or the test of this crate. 
#[macro_export]
macro_rules! u8bits {
    // bit range write
    (@field $(#[$attr:meta])* $t:ty, $id:ident:
     w $byte:expr, $lsb:expr, $msb:expr;) => {
        paste::item! {
            #[inline]
            $(#[$attr])*
            pub fn [<set_ $id>](&mut self, value: $t) {
                ::u8bits::set_bit_range(&mut self.0[$byte], $lsb, $msb, value.into())
            }
        }
    };
    // bit range read, returns $t
    (@field $(#[$attr:meta])* $t:ty, $id:ident:
     r $byte:expr, $lsb:expr, $msb:expr;) => {
        paste::item! {
            #[inline]
            $(#[$attr])*
            pub fn [<get_ $id>](&self) -> $t {
                ::u8bits::get_bit_range(&self.0[$byte], $lsb, $msb).into()
            }
        }
    };
    // bit range read and write
    (@field $(#[$attr:meta])* $t:ty, $id:ident:
     rw $byte:expr, $lsb:expr, $msb:expr;) => {
        u8bits!{@field $(#[$attr])* $t, $id: r $byte, $lsb, $msb;}
        u8bits!{@field $(#[$attr])* $t, $id: w $byte, $lsb, $msb;}
    };
    // single bit, write
    (@field $(#[$attr:meta])* $id:ident: w $byte:expr, $bit:expr;) => {
        paste::item! {
            #[inline]
            $(#[$attr])* 
            pub fn [<set_ $id>](&mut self, value: bool) {
                ::u8bits::set_bit(&mut self.0[$byte], $bit, value)
            }
        }
    };
    // single bit, read
    (@field $(#[$attr:meta])* $id:ident: r $byte:expr, $bit:expr;) => {
        paste::item! {
            #[inline]
            $(#[$attr])*
            pub fn [<get_ $id>](&self) -> bool {
                ::u8bits::get_bit(&self.0[$byte], $bit)
            }
        }
    };
    // single bit, read and write
    (@field $(#[$attr:meta])* $id:ident: rw $byte:expr, $bit:expr;) => {
        u8bits!{@field $(#[$attr])* $id: r $byte, $bit;}
        u8bits!{@field $(#[$attr])* $id: w $byte, $bit;}
    };
    // iteration, first is bit range
    ( $(#[$attr:meta])* $t:ty, $id:ident: $f:ident $($expr:expr),+; $($rest:tt)*) => {
        u8bits!{@field $(#[$attr])* $t, $id: $f $($expr),+;}
        u8bits!{$($rest)*}
    };
    // iteration, first is single bit
    ( $(#[$attr:meta])* $id:ident: $f:ident $($expr:expr),+; $($rest:tt)*) => {
        u8bits!{@field $(#[$attr])* $id: $f $($expr),+;}
        u8bits!{$($rest)*}
    };
    () => {};
}

