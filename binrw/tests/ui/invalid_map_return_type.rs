#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Foo {
    #[br(map = |_| 0u8)]
    a: i32,
}

fn main() {}
