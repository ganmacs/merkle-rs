#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Token<T> {
    raw: T,
    value: i64,
}

impl<T> Token<T> {
    pub fn new(v: T) -> Self
    where
        T: Into<i64> + Clone,
    {
        Token {
            raw: v.clone(),
            value: v.clone().into(),
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[test]
fn test_integer_cmp() {
    let a1 = Token::new(1);
    let a1_1 = Token::new(1);
    let a2 = Token::new(2);

    assert!(a2 > a1);
    assert!(a1_1 == a1);
}

#[test]
fn test_value() {
    let a1 = Token::new(1);
    let a2 = Token::new(2);

    assert!(a2.value() > a1.value());
}
