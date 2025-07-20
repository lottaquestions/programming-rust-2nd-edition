mod my_myascii {
    /// An ASCII encoded string
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii(
        // This must hold only well-formed ASCII text:
        // bytes from `0`` to `0x7f`
        Vec<u8>,
    );

    impl Ascii {
        /// Create an `Ascii` from the ASCII text in `bytes`. Return a
        /// `NotAsciiError` error if `bytes` contains any non-ASCII
        /// characters.
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }
    }
    // When conversion fails, we give back the vector we could not convert.
    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    // Safe, effecient conversion done using unsafe code
    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> String {
            // If this module has no  bugs, this is safe, because
            // well-formed ASCII text is also well-formed UTF-8.

            unsafe {
                String::from_utf8_unchecked(ascii.0)
            }
        }
    }
}

use my_myascii::Ascii;
fn main() {
    let bytes: Vec<u8> = b"ASCII and ye shall receive".to_vec();

    // This call entails no allocation or text copies, just a scan
    let ascii: Ascii = Ascii::from_bytes(bytes).unwrap();

    // This call is zero cost: no allocation, copies or scans.
    let string = String::from(ascii);
    assert_eq!(string, "ASCII and ye shall receive");
}
