use std::cmp::Ordering;

pub trait Token<T> {
    fn value(&self) -> u64;
    fn midpoint(&self, other: &Self) -> Self
    where
        Self: Token<T> + Sized;
}

impl<T> PartialOrd for Token<T> {
    fn partial_cmp(&self, other: &Token<T>) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl<T> PartialEq for Token<T> {
    fn eq(&self, other: &Token<T>) -> bool {
        self.value() == other.value()
    }
}

#[derive(Clone, Debug)]
pub struct IntegerToken {
    value: u64,
}

impl IntegerToken {
    pub fn new(v: u64) -> Self {
        IntegerToken { value: v }
    }
}

impl Token<u64> for IntegerToken {
    fn value(&self) -> u64 {
        self.value as u64
    }

    fn midpoint(&self, other: &Self) -> Self
    where
        Self: Token<u64> + Sized,
    {
        IntegerToken { value: (self.value + other.value()) / 2 }
    }
}
