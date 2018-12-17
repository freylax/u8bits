#![no_std]
//! A macro to generate getters and setters on structs which represent an array of bytes

extern crate paste;
#[allow(unused_imports)]
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
#[allow(unused_imports)]
use num_traits::FromPrimitive;

#[allow(dead_code)]
fn set_bit_range(b: &mut u8, lsb: u8, msb: u8, value: u8) {
    let mask: u8 = !0u8 << (7 - msb) >> (7 - msb + lsb) << lsb;
    *b &= !mask;
    *b |= (value << lsb) & mask;
}
#[allow(dead_code)]
fn get_bit_range(b: &u8, lsb: u8, msb: u8) -> u8 {
    let mask: u8 = !0u8 << (7 - msb) >> (7 - msb + lsb) << lsb;
    (*b & mask) >> lsb
}
#[allow(dead_code)]
fn set_bit(b: &mut u8, pos: u8, value: bool) {
    let mask: u8 = !1u8 << pos;
    *b &= !mask; if value { *b |= mask; }
}
#[allow(dead_code)]
fn get_bit(b: &u8, pos: u8) -> bool {
    let mask: u8 = !1u8 << pos;
    (*b & mask) == mask
}

#[macro_export(local_inner_macros)]
macro_rules! u8bits {
    // bit range write
    (@field $t:ty, $id:ident: w $byte:expr, $lsb:expr, $msb:expr;) => {
        paste::item! {
            fn [<set_ $id>](&mut self, value: $t) {
                set_bit_range(&mut self.0[$byte], $lsb, $msb, value as u8)
            }
        }
    };
    // bit range Read, returns Option<$t>
    (@field $t:ty, $id:ident: R $byte:expr, $lsb:expr, $msb:expr;) => {
        paste::item! {
            fn [<get_ $id>](&mut self) -> Option<$t> {
                $t::from_u8( get_bit_range(&mut self.0[$byte], $lsb, $msb))
            }
        }
    };
    // bit range read, returns $t
    (@field $t:ty, $id:ident: r $byte:expr, $lsb:expr, $msb:expr;) => {
        paste::item! {
            fn [<get_ $id>](&mut self) -> $t {
                get_bit_range(&mut self.0[$byte], $lsb, $msb) as $t
            }
        }
    };
    // bit range read and write
    (@field $t:ty, $id:ident: rw $byte:expr, $lsb:expr, $msb:expr;) => {
        u8bits!{@field $t, $id: r $byte, $lsb, $msb;}
        u8bits!{@field $t, $id: w $byte, $lsb, $msb;}
    };
    // bit range Read and write
    (@field $t:ty, $id:ident: Rw $byte:expr, $lsb:expr, $msb:expr;) => {
        u8bits!{@field $t, $id: R $byte, $lsb, $msb;}
        u8bits!{@field $t, $id: w $byte, $lsb, $msb;}
    };
    // bit range, default -> byte=0
    (@field $t:ty, $id:ident: $f:ident $lsb:expr, $msb:expr;) => {
        u8bits!{@field $t, $id: $f 0, $lsb, $msb;}
    };
    // single bit, write
    (@field $id:ident: w $byte:expr, $bit:expr;) => {
        paste::item! {
            fn [<set_ $id>](&mut self, value: bool) {
                set_bit(&mut self.0[$byte], $bit, value)
            }
        }
    };
    // single bit, read
    (@field $id:ident: r $byte:expr, $bit:expr;) => {
        paste::item! {
            fn [<get_ $id>](&mut self) -> bool {
                get_bit(&mut self.0[$byte], $bit)
            }
        }
    };
    // single bit, read and write
    (@field $id:ident: rw
     $byte:expr, $bit:expr;) => {
        u8bits!{@field $id: r $byte, $bit;}
        u8bits!{@field $id: w $byte, $bit;}
    };
    // single bit, default -> byte=0 
    (@field $id:ident: $f:ident $bit:expr;) =>
    { u8bits!{@field $id: $f 0, $bit;} };
    // iteration, first is bit range
    ( $t:ty, $id:ident: $f:ident $($expr:expr),+; $($rest:tt)*) => {
        u8bits!{@field $t, $id: $f $($expr),+;}
        u8bits!{$($rest)*}
    };
    // iteration, first is single bit
    ( $id:ident: $f:ident $($expr:expr),+; $($rest:tt)*) => {
        u8bits!{@field $id: $f $($expr),+;}
        u8bits!{$($rest)*}
    };
    () => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
