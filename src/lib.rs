pub mod parser;


#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use crate::parser;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_uint8() {
        assert_eq!(parser::uint8_t(b"\x04\xA5"), Ok((&b""[..], 0xA5)));
        assert_eq!(parser::uint8_t(b"\x04\xA7asd"), Ok((&b"asd"[..], 0xA7)));
        assert_eq!(parser::uint8_t(b"\x04"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::uint8_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_uint16() {
        assert_eq!(parser::uint16_t(b"\x05\x00\xA5"), Ok((&b""[..], 165u16)));
        assert_eq!(parser::uint16_t(b"\x05\x10\xA6"), Ok((&b""[..], 4262u16)));
        assert_eq!(parser::uint16_t(b"\x05\x00\xA7asd"), Ok((&b"asd"[..], 167u16)));
        assert_eq!(parser::uint16_t(b"\x05"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::uint16_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_uint32() {
        assert_eq!(parser::uint32_t(b"\x06\x00\x00\x00\xA5"), Ok((&b""[..], 165u32)));
        assert_eq!(parser::uint32_t(b"\x06\x00\x00\x10\xA6"), Ok((&b""[..], 4262u32)));
        assert_eq!(parser::uint32_t(b"\x06\x00\x00\x00\xA7asd"), Ok((&b"asd"[..], 167u32)));
        assert_eq!(parser::uint32_t(b"\x06"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::uint32_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_uint64() {
        assert_eq!(parser::uint64_t(b"\x07\x00\x00\x00\x00\x00\x00\x00\xA5"), Ok((&b""[..], 165u64)));
        assert_eq!(parser::uint64_t(b"\x07\x00\x00\x00\x00\x00\x00\x10\xA6"), Ok((&b""[..], 4262u64)));
        assert_eq!(parser::uint64_t(b"\x07\x00\x00\x00\x00\x00\x00\x00\xA7asd"), Ok((&b"asd"[..], 167u64)));
        assert_eq!(parser::uint64_t(b"\x07"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::uint64_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_int8() {
        assert_eq!(parser::int8_t(b"\x08\x00"), Ok((&b""[..],  0i8)));
        assert_eq!(parser::int8_t(b"\x08\x01"), Ok((&b""[..],  1i8)));
        assert_eq!(parser::int8_t(b"\x08\xff"), Ok((&b""[..], -1i8)));
        assert_eq!(parser::int8_t(b"\x08\xfe"), Ok((&b""[..], -2i8)));
        assert_eq!(parser::int8_t(b"\x08\x01asd"), Ok((&b"asd"[..], 1i8)));
        assert_eq!(parser::int8_t(b"\x08"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::int8_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_int16() {
        assert_eq!(parser::int16_t(b"\x09\x00\x00"), Ok((&b""[..],  0i16)));
        assert_eq!(parser::int16_t(b"\x09\x00\x01"), Ok((&b""[..],  1i16)));
        assert_eq!(parser::int16_t(b"\x09\xff\xff"), Ok((&b""[..], -1i16)));
        assert_eq!(parser::int16_t(b"\x09\xff\xfe"), Ok((&b""[..], -2i16)));
        assert_eq!(parser::int16_t(b"\x09\x00\x01asd"), Ok((&b"asd"[..], 1i16)));
        assert_eq!(parser::int16_t(b"\x09"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::int16_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_int32() {
        assert_eq!(parser::int32_t(b"\x0a\x00\x00\x00\x00"), Ok((&b""[..],  0i32)));
        assert_eq!(parser::int32_t(b"\x0a\x00\x00\x00\x01"), Ok((&b""[..],  1i32)));
        assert_eq!(parser::int32_t(b"\x0a\xff\xff\xff\xff"), Ok((&b""[..], -1i32)));
        assert_eq!(parser::int32_t(b"\x0a\xff\xff\xff\xfe"), Ok((&b""[..], -2i32)));
        assert_eq!(parser::int32_t(b"\x0a\x00\x00\x00\x01asd"), Ok((&b"asd"[..], 1i32)));
        assert_eq!(parser::int32_t(b"\x0a"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::int32_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_int64() {
        assert_eq!(parser::int64_t(b"\x0b\x00\x00\x00\x00\x00\x00\x00\x00"), Ok((&b""[..],  0i64)));
        assert_eq!(parser::int64_t(b"\x0b\x00\x00\x00\x00\x00\x00\x00\x01"), Ok((&b""[..],  1i64)));
        assert_eq!(parser::int64_t(b"\x0b\xff\xff\xff\xff\xff\xff\xff\xff"), Ok((&b""[..], -1i64)));
        assert_eq!(parser::int64_t(b"\x0b\xff\xff\xff\xff\xff\xff\xff\xfe"), Ok((&b""[..], -2i64)));
        assert_eq!(parser::int64_t(b"\x0b\x00\x00\x00\x00\x00\x00\x00\x01asd"), Ok((&b"asd"[..], 1i64)));
        assert_eq!(parser::int64_t(b"\x0b"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::int64_t(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_float32() {
        // https://baseconvert.com/ieee-754-floating-point
        assert_eq!(parser::float32(b"\x0c\x00\x00\x00\x00"), Ok((&b""[..],  0.0f32)));
        assert_eq!(parser::float32(b"\x0c\x3f\x80\x00\x00"), Ok((&b""[..],  1f32)));
        assert_eq!(parser::float32(b"\x0c\xbf\x80\x00\x00"), Ok((&b""[..], -1f32)));
        assert_eq!(parser::float32(b"\x0c\xc0\x00\x00\x00"), Ok((&b""[..], -2f32)));
        assert_eq!(parser::float32(b"\x0c\x7f\x61\xb1\xe6"), Ok((&b""[..], 3e+38f32)));
        assert_eq!(parser::float32(b"\x0c\x7f\x7f\xff\xff"), Ok((&b""[..], 3.4028234663852886e+38f32)));

        assert_eq!(parser::float32(b"\x0c\x7f\x80\x00\x00"), Ok((&b""[..], f32::INFINITY)));
        assert_eq!(parser::float32(b"\x0c\xff\x80\x00\x00"), Ok((&b""[..], f32::NEG_INFINITY)));

        assert_eq!(0.1f32, 0.100000001490116119384765625f32);
        assert_eq!(parser::float32(b"\x0c\x3d\xcc\xcc\xcd"), Ok((&b""[..], 0.1f32)));
        assert_eq!(parser::float32(b"\x0c\x3d\xcc\xcc\xcd"), Ok((&b""[..], 0.100000001490116119384765625f32)));

        assert_eq!(3.3f32, 3.2999999523162841796875f32);
        assert_eq!(parser::float32(b"\x0c\x40\x53\x33\x33"), Ok((&b""[..], 3.3f32)));
        assert_eq!(parser::float32(b"\x0c\x40\x53\x33\x33"), Ok((&b""[..], 3.2999999523162841796875f32)));

        let (_, nan) = parser::float32(b"\x0c\x7f\xc0\x00\x00").unwrap();
        assert!(nan.is_nan());
        let (_, nan) = parser::float32(b"\x0c\x7f\xcf\xff\xff").unwrap();
        assert!(nan.is_nan());
        let (_, nan) = parser::float32(b"\x0c\xff\xff\xff\xff").unwrap();
        assert!(nan.is_nan());

        assert_eq!(parser::float32(b"\x0c\x3f\x80\x00\x00asd"), Ok((&b"asd"[..], 1f32)));
        assert_eq!(parser::float32(b"\x0c"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::float32(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }

    #[test]
    fn parse_float64() {
        // https://baseconvert.com/ieee-754-floating-point
        assert_eq!(parser::float64(b"\x0d\x00\x00\x00\x00\x00\x00\x00\x00"), Ok((&b""[..],  0.0f64)));
        assert_eq!(parser::float64(b"\x0d\x3f\xf0\x00\x00\x00\x00\x00\x00"), Ok((&b""[..],  1f64)));
        assert_eq!(parser::float64(b"\x0d\xbf\xf0\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], -1f64)));
        assert_eq!(parser::float64(b"\x0d\xc0\x00\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], -2f64)));
        assert_eq!(parser::float64(b"\x0d\x47\xd2\xce\xd3\x2a\x16\xa1\xb1"), Ok((&b""[..], 1e38f64)));
        assert_eq!(parser::float64(b"\x0d\x47\xef\xff\xff\xe0\x00\x00\x00"), Ok((&b""[..], 3.4028234663852886e+38f64)));
        assert_eq!(parser::float64(b"\x0d\x7f\xef\xff\xff\xff\xff\xff\xff"), Ok((&b""[..], 1.7976931348623157e+308f64)));

        assert_eq!(parser::float64(b"\x0d\x7f\xf0\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], f64::INFINITY)));
        assert_eq!(parser::float64(b"\x0d\xff\xf0\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], f64::NEG_INFINITY)));

        assert_eq!(0.1f64, 0.1000000000000000055511151231257827021181583404541015625f64);
        assert_eq!(parser::float64(b"\x0d\x3f\xb9\x99\x99\x99\x99\x99\x9a"), Ok((&b""[..], 0.1f64)));
        assert_eq!(parser::float64(b"\x0d\x3f\xb9\x99\x99\x99\x99\x99\x9a"), Ok((&b""[..], 0.1000000000000000055511151231257827021181583404541015625f64)));

        assert_eq!(3.3f64, 3.29999999999999982236431605997495353221893310546875f64);
        assert_eq!(parser::float64(b"\x0d\x40\x0a\x66\x66\x66\x66\x66\x66"), Ok((&b""[..], 3.3f64)));
        assert_eq!(parser::float64(b"\x0d\x40\x0a\x66\x66\x66\x66\x66\x66"), Ok((&b""[..], 3.29999999999999982236431605997495353221893310546875f64)));
        
        let (_, nan) = parser::float64(b"\x0d\x7f\xf8\x00\x00\x00\x00\x00\x00").unwrap();
        assert!(nan.is_nan());
        let (_, nan) = parser::float64(b"\x0d\x7f\xf8\xff\xff\xff\xff\xff\xff").unwrap();
        assert!(nan.is_nan());
        let (_, nan) = parser::float64(b"\x0d\xff\xff\xff\xff\xff\xff\xff\xff").unwrap();
        assert!(nan.is_nan());

        assert_eq!(parser::float64(b"\x0d\x3f\xf0\x00\x00\x00\x00\x00\x00asd"), Ok((&b"asd"[..], 1f64)));
        assert_eq!(parser::float64(b"\x0d"), Err(nom::Err::Error((&[][..], ErrorKind::Eof))));
        assert_eq!(parser::float64(b"\x00"), Err(nom::Err::Error((&b"\x00"[..], ErrorKind::Tag))));
    }
}