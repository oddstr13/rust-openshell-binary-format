use std::ops::{AddAssign, MulAssign, Neg};

use core::str::from_utf8;

use serde::Deserialize;
use serde::de::{
	self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
	VariantAccess, Visitor,
};

use crate::error::{Error, Result};

pub struct Deserializer<'de> {
	// This string starts with the input data and characters are truncated off
	// the beginning as data is parsed.
	input: &'de [u8],
}

impl<'de> Deserializer<'de> {
	// By convention, `Deserializer` constructors are named like `from_xyz`.
	// That way basic use cases are satisfied by something like
	// `serde_json::from_str(...)` while advanced use cases that require a
	// deserializer can make one with `serde_json::Deserializer::from_str(...)`.
	pub fn from_bytes(input: &'de [u8]) -> Self {
		Deserializer { input }
	}
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
T: Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_bytes(s);
	let t = T::deserialize(&mut deserializer)?;
	if deserializer.input.is_empty() {
		return Ok(t);
	} else {
		return Err(Error::TrailingCharacters);
	}
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> Deserializer<'de> {
	// Look at the first character in the input without consuming it.
	#[inline(always)]
	fn peek(&mut self) -> Result<&u8> {
		return self.input.first().ok_or(Error::Eof);
	}

	#[inline(always)]
	fn consume(&mut self, num: usize) {
		self.input = &self.input[num..];
	}

	#[inline(always)]
	fn advance(&mut self) {
		self.consume(1);
	}

	#[inline(always)]
	fn has(&mut self, num: usize) -> Result<()> {
		if self.input.len() >= num {
			return Ok(());
		} else {
			return Err(Error::Eof);
		}
	}

	// Consume the first character in the input.
	fn next_byte(&mut self) -> Result<&u8> {
		let ch = self.peek()?;
		self.advance();
		return Ok(ch);
	}


	fn peek_is_string(&mut self) -> Result<bool> {
		let ch = self.peek()?;
		return Ok(
			ch & 0b_1100_0000__u8 == 0x80__u8
			|| ch == &0xc4
			|| ch == &0xc5
			|| ch == &0xc6
			|| ch == &0xc7
		)
	}


	// Parse the JSON identifier `true` or `false`.
	fn parse_bool(&mut self) -> Result<bool> {
		let ch = self.peek()?;

		if ch == &0x11__u8 {
			self.advance();
			return Ok(true);
		} else if ch == &0x10__u8 {
			self.advance();
			return Ok(false);
		} else {
			return Err(Error::ExpectedBoolean);
		}
	}


	/*
	// Parse a group of decimal digits as an unsigned integer of type T.
	//
	// This implementation is a bit too lenient, for example `001` is not
	// allowed in JSON. Also the various arithmetic operations can overflow and
	// panic or return bogus data. But it is good enough for example code!
	fn parse_unsigned<T>(&mut self) -> Result<T>
	where
	T: AddAssign<T> + MulAssign<T> + From<u8>,
	{
		let mut int = match self.next_char()? {
			ch @ '0'..='9' => T::from(ch as u8 - b'0'),
			_ => {
				return Err(Error::ExpectedInteger);
			}
		};
		loop {
			match self.input.chars().next() {
				Some(ch @ '0'..='9') => {
					self.input = &self.input[1..];
					int *= T::from(10);
					int += T::from(ch as u8 - b'0');
				}
				_ => {
					return Ok(int);
				}
			}
		}
	}
	*/

	/*
	// Parse a possible minus sign followed by a group of decimal digits as a
	// signed integer of type T.
	fn parse_signed<T>(&mut self) -> Result<T>
	where
	T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
	{
		// Optional minus sign, delegate to `parse_unsigned`, negate if negative.
		unimplemented!()
	}
	*/

	fn _parse_u8(&mut self) -> Result<u8> {
		return Ok(self.next_byte()?.to_owned());
	}

	fn _parse_u16(&mut self) -> Result<u16> {
		self.has(2)?;
		let mut res: u16 = 0;
		res |= (self.next_byte()?.to_owned() as u16) << 8;
		res |= self.next_byte()?.to_owned() as u16;
		return Ok(res);
	}

	fn _parse_u32(&mut self) -> Result<u32> {
		self.has(4)?;
		let mut res: u32 = 0;
		res |= (self.next_byte()?.to_owned() as u32) << 24;
		res |= (self.next_byte()?.to_owned() as u32) << 16;
		res |= (self.next_byte()?.to_owned() as u32) << 8;
		res |= self.next_byte()?.to_owned() as u32;
		return Ok(res);
	}

	fn _parse_u64(&mut self) -> Result<u64> {
		self.has(8)?;
		let mut res: u64 = 0;
		res |= (self.next_byte()?.to_owned() as u64) << 56;
		res |= (self.next_byte()?.to_owned() as u64) << 48;
		res |= (self.next_byte()?.to_owned() as u64) << 40;
		res |= (self.next_byte()?.to_owned() as u64) << 32;
		res |= (self.next_byte()?.to_owned() as u64) << 24;
		res |= (self.next_byte()?.to_owned() as u64) << 16;
		res |= (self.next_byte()?.to_owned() as u64) << 8;
		res |= self.next_byte()?.to_owned() as u64;
		return Ok(res);
	}

	fn parse_string(&mut self) -> Result<&'de str> {

		let ch = self.peek()?;
		let len: usize;

		if ch & 0b_1100_0000__u8 == 0x80__u8 {
			self.advance();
			len = (ch & 0b_0011_1111__u8) as usize;

		} else if ch == &0xc4 {
			self.advance();
			len = self._parse_u8()? as usize;

		} else if ch == &0xc5 {
			self.advance();
			len = self._parse_u16()? as usize;

		} else if ch == &0xc6 {
			self.advance();
			len = self._parse_u32()? as usize;

		} else if ch == &0xc7 {
			self.advance();
			let i =self._parse_u64()?;
			if i > usize::MAX as u64 {
				return Err(Error::TooLarge);
			}
			len = i as usize;

		} else {
			return Err(Error::ExpectedString);
		}

		self.has(len)?;

		let res = from_utf8(&self.input[..len]).or(Err(Error::Syntax))?;

		self.consume(len);

		return Ok(res);
	}
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
	type Error = Error;

	// Look at the input data to decide what Serde data model type to
	// deserialize as. Not all data formats are able to support this operation.
	// Formats that support `deserialize_any` are known as self-describing.
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		let ch = self.peek()?;
		if ch == & 0x11__u8 || ch == & 0x10__u8 {
			return self.deserialize_bool(visitor);
		} else {
			return Err(Error::Syntax);
		}
		/*
		 {
			'n' => self.deserialize_unit(visitor),
			't' | 'f' => self.deserialize_bool(visitor),
			'"' => self.deserialize_str(visitor),
			'0'..='9' => self.deserialize_u64(visitor),
			'-' => self.deserialize_i64(visitor),
			'[' => self.deserialize_seq(visitor),
			'{' => self.deserialize_map(visitor),
			_ => Err(Error::Syntax),
		}
		*/
	}

	// Uses the `parse_bool` parsing function defined above to read the JSON
	// identifier `true` or `false` from the input.
	//
	// Parsing refers to looking at the input and deciding that it contains the
	// JSON value `true` or `false`.
	//
	// Deserialization refers to mapping that JSON value into Serde's data
	// model by invoking one of the `Visitor` methods. In the case of JSON and
	// bool that mapping is straightforward so the distinction may seem silly,
	// but in other cases Deserializers sometimes perform non-obvious mappings.
	// For example the TOML format has a Datetime type and Serde's data model
	// does not. In the `toml` crate, a Datetime in the input is deserialized by
	// mapping it to a Serde data model "struct" type with a special name and a
	// single field containing the Datetime represented as a string.
	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		visitor.visit_bool(self.parse_bool()?)
	}


	// The `parse_signed` function is generic over the integer type `T` so here
	// it is invoked with `T=i8`. The next 8 methods are similar.
	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_i8(self.parse_signed()?)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_i16(self.parse_signed()?)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_i32(self.parse_signed()?)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_i64(self.parse_signed()?)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_u8(self.parse_unsigned()?)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_u16(self.parse_unsigned()?)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_u32(self.parse_unsigned()?)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
		// visitor.visit_u64(self.parse_unsigned()?)
	}

	// Float parsing is stupidly hard.
	fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
	}

	// Float parsing is stupidly hard.
	fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
	}


	// The `Serializer` implementation on the previous page serialized chars as
	// single-character strings so handle that representation here.
	fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		// Parse a string, check that it is one character, call `visit_char`.
		unimplemented!()
	}

	// Refer to the "Understanding deserializer lifetimes" page for information
	// about the three deserialization flavors of strings in Serde.
	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		visitor.visit_borrowed_str(self.parse_string()?)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	// The `Serializer` implementation on the previous page serialized byte
	// arrays as JSON arrays of bytes. Handle that representation here.
	fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
	}

	fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		unimplemented!()
	}

	// An absent optional is represented as the JSON `null` and a present
	// optional is represented as just the contained value.
	//
	// As commented in `Serializer` implementation, this is a lossy
	// representation. For example the values `Some(())` and `None` both
	// serialize as just `null`. Unfortunately this is typically what people
	// expect when working with JSON. Other formats are encouraged to behave
	// more intelligently if possible.
	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		if self.peek()? == &0x00__u8 {
			self.advance();
			visitor.visit_none()
		} else {
			visitor.visit_some(self)
		}
	}

	// In Serde, unit means an anonymous value containing no data.
	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		if self.peek()? == &0x00__u8 {
			self.advance();
			visitor.visit_unit()
		} else {
			Err(Error::ExpectedNull)
		}
	}

	// Unit struct means a named value containing no data.
	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_unit(visitor)
	}

	// As is done here, serializers are encouraged to treat newtype structs as
	// insignificant wrappers around the data they contain. That means not
	// parsing anything other than the contained value.
	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	// Deserialization of compound types like sequences and maps happens by
	// passing the visitor an "Access" object that gives it the ability to
	// iterate through the data contained in the sequence.
	fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		// Parse the opening bracket of the sequence.
		if self.next_byte()? == &0x15__u8 {
			// Give the visitor access to each element of the sequence.
			let value = visitor.visit_seq(CommaSeparated::new(&mut self))?;
			// Parse the closing bracket of the sequence.
			if self.next_byte()? == &0x17__u8 {
				Ok(value)
			} else {
				Err(Error::ExpectedSequenceEnd)
			}
		} else {
			Err(Error::ExpectedArray)
		}
	}

	// Tuples look just like sequences in JSON. Some formats may be able to
	// represent tuples more efficiently.
	//
	// As indicated by the length parameter, the `Deserialize` implementation
	// for a tuple in the Serde data model is required to know the length of the
	// tuple before even looking at the input data.
	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	// Tuple structs look just like sequences in JSON.
	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	// Much like `deserialize_seq` but calls the visitors `visit_map` method
	// with a `MapAccess` implementation, rather than the visitor's `visit_seq`
	// method with a `SeqAccess` implementation.
	fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		// Parse the opening brace of the map.
		if self.next_byte()? == &0x16__u8 {
			// Give the visitor access to each entry of the map.
			let value = visitor.visit_map(CommaSeparated::new(&mut self))?;
			// Parse the closing brace of the map.
			if self.next_byte()? == &0x17__u8 {
				Ok(value)
			} else {
				Err(Error::ExpectedSequenceEnd)
			}
		} else {
			Err(Error::ExpectedMap)
		}
	}

	// Structs look just like maps in JSON.
	//
	// Notice the `fields` parameter - a "struct" in the Serde data model means
	// that the `Deserialize` implementation is required to know what the fields
	// are before even looking at the input data. Any key-value pairing in which
	// the fields cannot be known ahead of time is probably a map.
	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		if self.peek_is_string()? {
			// Visit a unit variant.
			visitor.visit_enum(self.parse_string()?.into_deserializer())
		} else if self.next_byte()? == &0x16__u8 {
			// Visit a newtype variant, tuple variant, or struct variant.
			let value = visitor.visit_enum(Enum::new(self))?;
			// Parse the matching close brace.
			if self.next_byte()? == &0x17__u8 {
				Ok(value)
			} else {
				Err(Error::ExpectedSequenceEnd)
			}
		} else {
			Err(Error::ExpectedEnum)
		}
	}

	// An identifier in Serde is the type that identifies a field of a struct or
	// the variant of an enum. In JSON, struct fields and enum variants are
	// represented as strings. In other formats they may be represented as
	// numeric indices.
	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	// Like `deserialize_any` but indicates to the `Deserializer` that it makes
	// no difference which `Visitor` method is called because the data is
	// ignored.
	//
	// Some deserializers are able to implement this more efficiently than
	// `deserialize_any`, for example by rapidly skipping over matched
	// delimiters without paying close attention to the data in between.
	//
	// Some formats are not able to implement this at all. Formats that can
	// implement `deserialize_any` and `deserialize_ignored_any` are known as
	// self-describing.
	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
	where
	V: Visitor<'de>,
	{
		self.deserialize_any(visitor)
	}
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct CommaSeparated<'a, 'de: 'a> {
	de: &'a mut Deserializer<'de>,
	first: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
		CommaSeparated {
			de,
			first: true,
		}
	}
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
	type Error = Error;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
	where
	T: DeserializeSeed<'de>,
	{
		// Check if there are no more elements.
		if self.de.peek()? == &0x17__u8 {
			return Ok(None);
		}
		/*
		// Comma is required before every element except the first.
		if !self.first && self.de.next_byte()? != ',' {
			return Err(Error::ExpectedArrayComma);
		}
		self.first = false;
		*/
		// Deserialize an array element.
		seed.deserialize(&mut *self.de).map(Some)
	}
}

// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
	where
	K: DeserializeSeed<'de>,
	{
		// Check if there are no more entries.
		if self.de.peek()? == &0x17__u8 {
			return Ok(None);
		}
		/*
		// Comma is required before every entry except the first.
		if !self.first && self.de.next_byte()? != ',' {
			return Err(Error::ExpectedMapComma);
		}
		self.first = false;
		*/
		// Deserialize a map key.
		seed.deserialize(&mut *self.de).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
	where
	V: DeserializeSeed<'de>,
	{
		// It doesn't make a difference whether the colon is parsed at the end
		// of `next_key_seed` or at the beginning of `next_value_seed`. In this
		// case the code is a bit simpler having it here.
		/*
		if self.de.next_byte()? != ':' {
			return Err(Error::ExpectedMapColon);
		}
		*/
		// Deserialize a map value.
		seed.deserialize(&mut *self.de)
	}
}

struct Enum<'a, 'de: 'a> {
	de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
	fn new(de: &'a mut Deserializer<'de>) -> Self {
		Enum { de }
	}
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
	where
	V: DeserializeSeed<'de>,
	{
		// The `deserialize_enum` method parsed a `{` character so we are
			// currently inside of a map. The seed will be deserializing itself from
			// the key of the map.
			let val = seed.deserialize(&mut *self.de)?;
			Ok((val, self))
			/*
			// Parse the colon separating map key from value.
			if self.de.next_byte()? == ':' {
				Ok((val, self))
			} else {
				Err(Error::ExpectedMapColon)
			}
			*/
		}
	}

	// `VariantAccess` is provided to the `Visitor` to give it the ability to see
	// the content of the single variant that it decided to deserialize.
	impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
		type Error = Error;

		// If the `Visitor` expected this variant to be a unit variant, the input
		// should have been the plain string case handled in `deserialize_enum`.
		fn unit_variant(self) -> Result<()> {
			Err(Error::ExpectedString)
		}

		// Newtype variants are represented in JSON as `{ NAME: VALUE }` so
		// deserialize the value here.
		fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
		where
		T: DeserializeSeed<'de>,
		{
			seed.deserialize(self.de)
		}

		// Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
		// deserialize the sequence of data here.
		fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
		where
		V: Visitor<'de>,
		{
			de::Deserializer::deserialize_seq(self.de, visitor)
		}

		// Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
		// deserialize the inner map here.
		fn struct_variant<V>(
			self,
			_fields: &'static [&'static str],
			visitor: V,
		) -> Result<V::Value>
		where
		V: Visitor<'de>,
		{
			de::Deserializer::deserialize_map(self.de, visitor)
		}
	}

	////////////////////////////////////////////////////////////////////////////////
	/*
	#[test]
	fn test_struct() {
		#[derive(Deserialize, PartialEq, Debug)]
		struct Test {
			int: u32,
			seq: Vec<String>,
		}

		let j = r#"{"int":1,"seq":["a","b"]}"#;
		let expected = Test {
			int: 1,
			seq: vec!["a".to_owned(), "b".to_owned()],
		};
		assert_eq!(expected, from_str(j).unwrap());
	}

	#[test]
	fn test_enum() {
		#[derive(Deserialize, PartialEq, Debug)]
		enum E {
			Unit,
			Newtype(u32),
			Tuple(u32, u32),
			Struct { a: u32 },
		}

		let j = r#""Unit""#;
		let expected = E::Unit;
		assert_eq!(expected, from_str(j).unwrap());

		let j = r#"{"Newtype":1}"#;
		let expected = E::Newtype(1);
		assert_eq!(expected, from_str(j).unwrap());

		let j = r#"{"Tuple":[1,2]}"#;
		let expected = E::Tuple(1, 2);
		assert_eq!(expected, from_str(j).unwrap());

		let j = r#"{"Struct":{"a":1}}"#;
		let expected = E::Struct { a: 1 };
		assert_eq!(expected, from_str(j).unwrap());
	}
	*/