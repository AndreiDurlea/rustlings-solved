fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn literals() {
        // TODO: Replace each `???` with a literal written in the notation
        // described in the comment. Don't just write the decimal number!

        // 255 in hexadecimal
        let hex = ???;
        assert_eq!(hex, 255);

        // 8 in octal
        let octal = ???;
        assert_eq!(octal, 8);

        // 10 in binary
        let binary = ???;
        assert_eq!(binary, 10);

        // The ASCII code of the letter 'A' as a byte literal
        let byte = ???;
        assert_eq!(byte, 65);

        // One million, using `_` to make it readable
        let million = ???;
        assert_eq!(million, 1000000);

        // 7 as an `i64` using a type suffix (no type annotation!)
        let suffixed = ???;
        assert_eq!(size_of_val(&suffixed), 8);
        assert_eq!(suffixed, 7);
    }
}
