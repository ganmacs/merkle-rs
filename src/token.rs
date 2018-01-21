use std::cmp::Ordering;

pub trait Token {
    type V;

    fn value(&self) -> u64;
    fn midpoint(&self, other: &Self) -> Self
    where
        Self: Sized;
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
