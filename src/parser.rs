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
use nom::character::complete::char as nom_char;
use nom::combinator::map_res;



pub fn uint8_t(input: &str) -> IResult<&str, u8> {
    preceded(
        nom_char('\x04'),
        be_u8
    )(input)
}

/*
pub fn parse(input: &str) -> IResult<&str, > {
    
}
*/