use std::ops::Range;
use token::{Token, IntegerToken};

pub trait Partitioner<T: Token<S>, S> {
    fn call(&self, range: &Range<T>) -> Option<(Range<T>, Range<T>)>;
}

pub struct IPartitioner {}

impl Partitioner<IntegerToken, u64> for IPartitioner {
    fn call(
        &self,
        range: &Range<IntegerToken>,
    ) -> Option<(Range<IntegerToken>, Range<IntegerToken>)> {
        let s = range.start.clone();
        let e = range.end.clone();
        let sv = s.value();
        let se = e.value();
        if sv == se {
            return None;
        }

        let mid = (sv + se) / 2;
        let l = Range {
            start: IntegerToken::new(sv),
            end: IntegerToken::new(mid),
        };
        let r = Range {
            start: IntegerToken::new(mid + 1),
            end: IntegerToken::new(se),
        };

        Some((l, r))
    }
}
