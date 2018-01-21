use digest::Digestible;

// #[derive(Clone, Debug)]
// XXX: HashMap?
pub struct RowHash<T, V: Digestible> {
    pub key: T,
    pub value: V,
}

impl<T, V: Digestible> RowHash<T, V> {
    pub fn new(key: T, value: V) -> Self {
        RowHash {
            key: key,
            value: value,
        }
    }
}
