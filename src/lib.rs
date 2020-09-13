mod de;
mod error;
mod ser;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};


#[cfg(test)]
mod tests {
    //use nom::error::ErrorKind;
    //use crate::parser;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /*
    #[test]
    fn parse_uint8() {
        assert_eq!(parser::uint8_t(b"\x04\xA5"), Ok((&b""[..], 0xA5)));
        assert_eq!(parser::uint8_t(b"\x04\xA7asd"), Ok((&b"asd"[..], 0xA7)));
        assert_eq!(parser::uint8_t(b"\x04"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::uint8_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }
    */
}
