use std::ops::Range;
use token::{Token, IntegerToken};

pub trait Partitioner {
    type Token: Token;

    fn call(&self, range: &Range<Self::Token>) -> Option<(Range<Self::Token>, Range<Self::Token>)>;
}

pub struct IPartitioner {}

impl Partitioner for IPartitioner {
    type Token = IntegerToken;

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
