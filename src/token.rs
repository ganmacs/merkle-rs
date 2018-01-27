use std::cmp::Ordering;
use std::fmt;

pub trait Token {
    type V;

    fn value(&self) -> u64;
    fn midpoint(&self, other: &Self) -> Self
    where
        Self: Sized;
}

impl<T> fmt::Debug for Token<V = T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?})", self.value())
    }
}

impl<T> PartialOrd for Token<V = T> {
    fn partial_cmp(&self, other: &Token<V = T>) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl<T> PartialEq for Token<V = T> {
    fn eq(&self, other: &Token<V = T>) -> bool {
        self.value() == other.value()
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct IntegerToken {
    value: u64,
}

impl IntegerToken {
    pub fn new(v: u64) -> Self {
        IntegerToken { value: v }
    }
}

impl Token for IntegerToken {
    type V = u64;

    fn value(&self) -> u64 {
        self.value as u64
    }

    fn midpoint(&self, other: &Self) -> Self {
        IntegerToken { value: (self.value + other.value()) / 2 }
    }
}

impl fmt::Debug for IntegerToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?})", self.value())
    }
}

#[test]
fn test_integer_cmp() {
    let a1 = IntegerToken { value: 1 };
    let a1_1 = IntegerToken { value: 1 };
    let a2 = IntegerToken { value: 2 };

    assert!(a2 > a1);
    assert!(a1_1 == a1);
}
