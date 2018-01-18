use digest::Digestible;

pub struct RowHash<T, S: Digestible> {
    key: T,
    value: S,
}

impl<T, S: Digestible> RowHash<T, S> {
    pub fn new(key: T, value: S) -> Self {
        RowHash {
            key: key,
            value: value,
        }
    }
}
