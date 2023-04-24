use std::collections::HashMap;
use std::fmt::Display;
use serde::Serializer;
use serde::ser::{Serialize, SerializeMap, SerializeStruct};

pub struct Mask {
    name: String,
    properties: HashMap<String, Mask>
}

pub enum MaskedWrapper<'a, S, InnerSerializer>
{
    MaskedPassThrough {
        serializer: S,
        inner_serializer: InnerSerializer,
        mask: &'a Mask
    },
    Null
}

impl<'a, S> SerializeStruct for MaskedWrapper<'a, S, S::SerializeStruct>
    where
        S: Serializer
{
    type Ok = <S as Serializer>::Ok;
    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        match self {
            MaskedWrapper::MaskedPassThrough { serializer, inner_serializer, mask } => {
                inner_serializer.skip_field(key)
            },
            MaskedWrapper::Null => Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, S> SerializeMap for MaskedWrapper<'a, S, S::SerializeMap>
    where
        S: Serializer
{
    type Ok = <S as Serializer>::Ok;
    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), Self::Error> where K: Serialize, V: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct MaskedSerializer<'a, S>
    where
        S: Serializer
{
    serializer: &'a mut S,
    mask: &'a Mask
}

impl<'a, S> MaskedSerializer<'a, S>
    where
        S: Serializer
{
    pub fn new(serializer: &'a mut S,
               mask: &'a Mask) -> Self {
        MaskedSerializer { serializer, mask }
    }
}

//impl<'a, S: 'a> Serializer for &'a mut MaskedSerializer<'a, S>
impl<'a, S: 'a> Serializer for MaskedSerializer<'a, S>
    where
        S: Serializer
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = MaskedWrapper<'a, S, S::SerializeMap>;
    type SerializeStruct = MaskedWrapper<'a, S, S::SerializeStruct>;
    type SerializeStructVariant = S::SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!() //self.serializer.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!() //self.serializer.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!() //self.serializer.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!() //self.serializer.serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!() //self.serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!() //self.serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!() //self.serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!() //self.serializer.serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!() //self.serializer.serialize_struct_variant(name, variant_index, variant, len)
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error> where I: IntoIterator, <I as IntoIterator>::Item: Serialize {
        todo!() //self.serializer.collect_seq(iter)
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error> where K: Serialize, V: Serialize, I: IntoIterator<Item=(K, V)> {
        todo!() //self.serializer.collect_map(iter)
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Display {
        todo!() //self.serializer.collect_str(value)
    }

    fn is_human_readable(&self) -> bool {
        todo!() //self.serializer.is_human_readable()
    }
}