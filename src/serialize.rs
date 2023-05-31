use std::collections::HashMap;
use std::fmt::Display;
use serde::Serializer;
use serde::ser::{Serialize, SerializeMap, SerializeStruct};

pub struct Mask {
    name: String,
    properties: HashMap<String, Mask>
}

pub struct MaskedSerializeStructWrapper<'a, S>
    where S: SerializeStruct
{
    struct_serializer: S,
    mask: &'a Mask
}

// Note, the key here is going to be wrapping the individually passed things.
// This is how we can track the state of the mask recursively

impl<S> SerializeStruct for MaskedSerializeStructWrapper<'_, S>
    where S: SerializeStruct
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        if self.mask.properties.contains_key(key) {
            self.struct_serializer.serialize_field()
        }

        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        if self.mask.properties.contains_key(key) {
            self.struct_serializer.skip_field(key)?
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.struct_serializer.end()
    }
}

pub struct MaskedSerializeMapWrapper<'a, S>
    where S: SerializeMap
{
    map_serializer: S,
    mask: &'a Mask
}

impl<S> SerializeMap for MaskedSerializeMapWrapper<'_, S>
    where S: SerializeMap
{
    type Ok = S::Ok;
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
{
    serializer: &'a mut S,
    mask: Mask
}

impl<'a, S> MaskedSerializer<'a, S>
{
    pub fn new(serializer: &'a mut S,
               mask: Mask) -> Self {
        MaskedSerializer { serializer, mask }
    }
}

impl<'a, S> Serializer for &'a mut MaskedSerializer<'a, S>
    where &'a mut S: Serializer
{
    type Ok = <&'a mut S as Serializer>::Ok;
    type Error = <&'a mut S as Serializer>::Error;

    type SerializeSeq = <&'a mut S as Serializer>::SerializeSeq;
    type SerializeTuple = <&'a mut S as Serializer>::SerializeTuple;
    type SerializeTupleStruct = <&'a mut S as Serializer>::SerializeTupleStruct;
    type SerializeTupleVariant = <&'a mut S as Serializer>::SerializeTupleVariant;
    type SerializeMap = MaskedSerializeMapWrapper<'a, <&'a mut S as Serializer>::SerializeMap>;
    type SerializeStruct = MaskedSerializeStructWrapper<'a, <&'a mut S as Serializer>::SerializeStruct>;
    type SerializeStructVariant = <&'a mut S as Serializer>::SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        self.serializer.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        self.serializer.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        self.serializer.serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serializer.serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let underlying_serializer = self.serializer.serialize_map(len)?;
        Ok(MaskedSerializeMapWrapper { map_serializer: underlying_serializer, mask: &self.mask })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        let underlying_serializer = self.serializer.serialize_struct(name, len)?;
        Ok(MaskedSerializeStructWrapper { struct_serializer: underlying_serializer, mask: &self.mask })
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serializer.serialize_struct_variant(name, variant_index, variant, len)
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error> where I: IntoIterator, <I as IntoIterator>::Item: Serialize {
        self.serializer.collect_seq(iter)
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error> where K: Serialize, V: Serialize, I: IntoIterator<Item=(K, V)> {
        self.serializer.collect_map(iter)
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Display {
        self.serializer.collect_str(value)
    }

    fn is_human_readable(&self) -> bool {
        self.serializer.is_human_readable()
    }
}
