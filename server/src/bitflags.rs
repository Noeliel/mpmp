pub trait BitFlag<T> {
    fn contains(&self, flag: T) -> bool;
    fn is(&self, flag: T) -> bool;
}

impl BitFlag<u32> for u32 {
    fn contains(&self, flag: u32) -> bool {
        (self & flag) == flag
    }

    fn is(&self, flag: u32) -> bool {
        *self == flag
    }
}
