#![no_std]
#![deny(
    //missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

use u8bits::u8bits;
use surjective_enum::From;

#[test]
fn test() {
    #[derive(From,PartialEq,Debug)]
    #[repr(u8)]
    enum Enu { Bar = 0b00, Foo = 0b01, Oof = 0b10, Rab = 0b11}
    
    let bar: Enu = 0u8.into();
    let _foo: u8 = Enu::Foo.into();
    let _x : u16 = 0u16.into();
    
    assert_eq!( Enu::Bar, bar);
    
    struct Stru( [u8;2]);
    impl Stru {
        u8bits! {
            /// foo
            foo: rw 0,4;
            /// Enum example
            Enu, enu: rw 0,0,1;
            u8,bar: rw 0,6,7;
            /// foobarcomment
            foobar: rw 1, 0;
        }
    }
    let mut s = Stru( [0u8;2]);
    s.set_foo( true);
    assert_eq!( true, s.get_foo());
    s.set_bar( 3);
    assert_eq!( 3, s.get_bar());
    s.set_enu( Enu::Oof);
    assert_eq!( Enu::Oof, s.get_enu());
    struct Bytes( [u8;2]);
    impl Bytes {
        u8bits! {
            /// foo is bit 4 of byte 0
            foo: rw 0,4;
            /// bar are bits 0..3 of byte 1
            u8, bar: rw 1,0,3;
        }
    }
    let mut b = Bytes( [0u8;2]);
    b.set_foo( true);
    assert_eq!( true, b.get_foo());
}


