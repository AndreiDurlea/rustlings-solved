fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn sizes() {
        assert_eq!(size_of::<u8>(), 1);
        assert_eq!(size_of::<i32>(), 4);
        assert_eq!(size_of::<u64>(), 8);
        assert_eq!(size_of::<i128>(), 16);
        assert_eq!(size_of::<f32>(), 4);
        assert_eq!(size_of::<f64>(), 8);
        assert_eq!(size_of::<bool>(), 1);
        // A `char` is a Unicode scalar value, not a single byte!
        assert_eq!(size_of::<char>(), 4);
        // The unit type has only one possible value, so it needs no memory.
        assert_eq!(size_of::<()>(), 0);
        // 10 elements * 2 bytes
        assert_eq!(size_of::<[u16; 10]>(), 20);
    }
}
