fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn literals() {
        // 255 in hexadecimal
        let hex = 0xff;
        assert_eq!(hex, 255);

        // 8 in octal
        let octal = 0o10;
        assert_eq!(octal, 8);

        // 10 in binary
        let binary = 0b1010;
        assert_eq!(binary, 10);

        // The ASCII code of the letter 'A' as a byte literal
        let byte = b'A';
        assert_eq!(byte, 65);

        // One million, using `_` to make it readable
        let million = 1_000_000;
        assert_eq!(million, 1000000);

        // 7 as an `i64` using a type suffix (no type annotation!)
        let suffixed = 7i64;
        assert_eq!(size_of_val(&suffixed), 8);
        assert_eq!(suffixed, 7);
    }
}
