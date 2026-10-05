fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn sizes() {
        // TODO: Replace each `???` with the size in bytes of the type.
        // `size_of::<T>()` returns how many bytes a value of type `T` needs.
        assert_eq!(size_of::<u8>(), ???);
        assert_eq!(size_of::<i32>(), ???);
        assert_eq!(size_of::<u64>(), ???);
        assert_eq!(size_of::<i128>(), ???);
        assert_eq!(size_of::<f32>(), ???);
        assert_eq!(size_of::<f64>(), ???);
        assert_eq!(size_of::<bool>(), ???);
        assert_eq!(size_of::<char>(), ???);
        assert_eq!(size_of::<()>(), ???);
        assert_eq!(size_of::<[u16; 10]>(), ???);
    }
}
