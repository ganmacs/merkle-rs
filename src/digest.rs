use ring;
use std::convert;
use ring::digest::{Context, SHA1};

pub trait Digestible {
    fn as_ref(&self) -> &[u8];
}

impl Digestible for Vec<u8> {
    fn as_ref(&self) -> &[u8] {
        convert::AsRef::as_ref(self)
    }
}

// XXX: digest function would be settable
pub fn digest<T: Digestible>(value: &T) -> Vec<u8> {
    ring::digest::digest(&SHA1, value.as_ref()).as_ref().into()
}

pub fn digest2<T: Digestible>(value1: &T, value2: &T) -> Vec<u8> {
    let mut ctx = Context::new(&SHA1);
    ctx.update(value1.as_ref());
    ctx.update(value2.as_ref());
    ctx.finish().as_ref().into()
}
