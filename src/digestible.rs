use ring;
use ring::digest::{Context, SHA1};

pub use ring::digest::Digest;

pub trait Digestible {
    fn digest(&self) -> Digest;
    fn is_same_digest<T: Digestible>(&self, other: &T) -> bool;
}

pub fn vector_digest(value: &Vec<u8>) -> Digest {
    ring::digest::digest(&SHA1, value)
}

// pub fn digest<T: Into<Vec<u8>>>(value: T) -> Digest {
//     ring::digest::digest(&SHA1, value.into().as_ref())
// }

pub fn concat_digest(d1: &Digest, d2: &Digest) -> Digest {
    let mut ctx = Context::new(&SHA1);
    ctx.update(d1.as_ref());
    ctx.update(d2.as_ref());
    ctx.finish()
}
