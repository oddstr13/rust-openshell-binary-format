#![allow(unused_imports)]

use nom::IResult;

use nom::number::complete::{
    be_u8,
    be_u16,
    be_u32,
    be_u64,
    
    be_i8,
    be_i16,
    be_i32,
    be_i64,
    
    be_f32,
    be_f64,
};

use nom::bytes::complete::{
    take,
    tag
};
use nom::sequence::preceded;
use nom::combinator::map_res;


pub fn uint8_t(input: &[u8]) -> IResult<&[u8], u8> {
    preceded(
        tag("\x04"),
        be_u8
    )(input)
}

pub fn uint16_t(input: &[u8]) -> IResult<&[u8], u16> {
    preceded(
        tag("\x05"),
        be_u16
    )(input)
}

pub fn uint32_t(input: &[u8]) -> IResult<&[u8], u32> {
    preceded(
        tag("\x06"),
        be_u32
    )(input)
}

pub fn uint64_t(input: &[u8]) -> IResult<&[u8], u64> {
    preceded(
        tag("\x07"),
        be_u64
    )(input)
}

pub fn int8_t(input: &[u8]) -> IResult<&[u8], i8> {
    preceded(
        tag("\x08"),
        be_i8
    )(input)
}

pub fn int16_t(input: &[u8]) -> IResult<&[u8], i16> {
    preceded(
        tag("\x09"),
        be_i16
    )(input)
}

pub fn int32_t(input: &[u8]) -> IResult<&[u8], i32> {
    preceded(
        tag("\x0a"),
        be_i32
    )(input)
}

pub fn int64_t(input: &[u8]) -> IResult<&[u8], i64> {
    preceded(
        tag("\x0b"),
        be_i64
    )(input)
}

pub fn float32(input: &[u8]) -> IResult<&[u8], f32> {
    preceded(
        tag("\x0c"),
        be_f32
    )(input)
}

pub fn float64(input: &[u8]) -> IResult<&[u8], f64> {
    preceded(
        tag("\x0d"),
        be_f64
    )(input)
}





//parse_be!(uint16_t, u8, b'\x05');

/*
pub fn parse(input: &str) -> IResult<&str, > {
    
}
*/