use calamine::DataType;
use calamine::Rows;
use conv::errors::UnwrapOrInf;
use conv::ApproxInto;
use conv::GeneralError;
use conv::ValueInto;
use serde::de::value::BorrowedStrDeserializer;
use serde::de::DeserializeSeed;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use std;
use std::iter::Peekable;
use std::marker::PhantomData;

// See csv deserializer for how to implement a Deserializer
// https://github.com/BurntSushi/rust-csv/blob/master/src/deserializer.rs

pub fn deserialize<'de, T: Deserialize<'de>>(rows: Rows<'de, DataType>) -> RowsDeserializeIter<T> {
    RowsDeserializeIter::new(rows)
}
pub struct RowsDeserializeIter<'de, T> {
    header: Vec<Option<&'de str>>,
    rows: Rows<'de, DataType>,
    _phantom: PhantomData<&'de ()>,
    _phantom_t: PhantomData<&'de T>,
}
impl<'de, T: Deserialize<'de>> RowsDeserializeIter<'de, T> {
    fn new(mut rows: Rows<'de, DataType>) -> Self {
        let header_row: &[DataType] = rows.next().unwrap_or(&[]);
        let header = header_row
            .iter()
            .map(|data_type: &DataType| data_type.get_string())
            .collect::<Vec<Option<&str>>>();

        RowsDeserializeIter {
            header,
            rows,
            _phantom: PhantomData,
            _phantom_t: PhantomData,
        }
    }
}
impl<'de, T: Deserialize<'de>> Iterator for RowsDeserializeIter<'de, T> {
    type Item = Result<T, serde::de::value::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(data_types) = self.rows.next() {
            let row_iter = data_types.iter().peekable();
            let header_iter = self.header.iter();
            Some(T::deserialize(&mut RowDeserializer {
                row_iter,
                header_iter,
            }))
        } else {
            None
        }
    }
}

struct RowDeserializer<'a, 'de> {
    header_iter: std::slice::Iter<'a, Option<&'de str>>,
    row_iter: Peekable<std::slice::Iter<'de, DataType>>,
}
fn conv_error_to_serde_error<T, E: Into<GeneralError<T>>>(error: E) -> serde::de::value::Error {
    let error: GeneralError<T> = error.into();
    match error {
        GeneralError::NegOverflow(_n) => serde::de::Error::custom("NegOverflow"),
        GeneralError::PosOverflow(_n) => serde::de::Error::custom("PosOverflow"),
        GeneralError::Unrepresentable(_n) => serde::de::Error::custom("Unrepresentable"),
    }
}
macro_rules! deserialize_int {
    ($method:ident, $visit:ident, $ty:ty) => {
        fn $method<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let n = match self.row_iter.next() {
                Some(&DataType::Bool(value)) => if value {1} else {0},
                Some(&DataType::Int(value)) => ValueInto::<$ty>::value_into(value)
                    .map_err(conv_error_to_serde_error::<i64, _>)?,
                Some(&DataType::Float(_)) => return Err(serde::de::Error::custom("Expected int, got float")),
                Some(&DataType::String(_)) => return Err(serde::de::Error::custom("Expected int, got string")),
                Some(&DataType::Error(_)) => return Err(serde::de::Error::custom("Expected int, got Error")),
                Some(&DataType::Empty) => return Err(serde::de::Error::custom("Expected int, got Empty")),
                None => return Err(serde::de::Error::custom("Empty cell instead of number")),
            };
            visitor.$visit(n)
        }
    }
}

impl<'a, 'de> Deserializer<'de> for &mut RowDeserializer<'a, 'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.row_iter.next() {
            Some(&DataType::Bool(value)) => visitor.visit_bool(value),
            Some(&DataType::Int(value)) => visitor.visit_bool(value != 0),
            Some(&DataType::Float(value)) => visitor.visit_bool(value != 0f64),
            //            Some(&DataType::Empty) | None => visitor.visit_bool(false),
            Some(_) => {
                let error: serde::de::value::Error = serde::de::Error::custom("expected bool cell");
                Err(error)
            }
            None => {
                let error: serde::de::value::Error =
                    serde::de::Error::custom("expected bool cell; got None");
                Err(error)
            }
        }
    }

    deserialize_int!(deserialize_i8, visit_i8, i8);
    deserialize_int!(deserialize_i16, visit_i16, i16);
    deserialize_int!(deserialize_i32, visit_i32, i32);
    deserialize_int!(deserialize_i64, visit_i64, i64);
    deserialize_int!(deserialize_u8, visit_u8, u8);
    deserialize_int!(deserialize_u16, visit_u16, u16);
    deserialize_int!(deserialize_u32, visit_u32, u32);
    deserialize_int!(deserialize_u64, visit_u64, u64);

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.row_iter.next() {
            Some(&DataType::Bool(_value)) => {
                Err(serde::de::Error::custom("expected float cell, got bool"))
            }
            Some(&DataType::Int(value)) => {
                visitor.visit_f32(ValueInto::value_into(value).unwrap_or_inf())
            }
            Some(&DataType::Float(value)) => {
                visitor.visit_f32(ApproxInto::approx_into(value).unwrap_or_inf())
            }
            //            Some(&DataType::Empty) | None => visitor.visit_bool(false),
            Some(_) => {
                let error: serde::de::value::Error =
                    serde::de::Error::custom("expected float cell");
                Err(error)
            }
            None => {
                let error: serde::de::value::Error =
                    serde::de::Error::custom("expected bool cell; got None");
                Err(error)
            }
        }
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.row_iter.next() {
            Some(&DataType::Bool(_value)) => {
                Err(serde::de::Error::custom("expected float cell, got bool"))
            }
            Some(&DataType::Int(value)) => {
                visitor.visit_f64(ValueInto::value_into(value).unwrap_or_inf())
            }
            Some(&DataType::Float(value)) => visitor.visit_f64(value),
            //            Some(&DataType::Empty) | None => visitor.visit_bool(false),
            Some(_) => {
                let error: serde::de::value::Error =
                    serde::de::Error::custom("expected float cell");
                Err(error)
            }
            None => {
                let error: serde::de::value::Error =
                    serde::de::Error::custom("expected bool cell; got None");
                Err(error)
            }
        }
    }
    fn deserialize_char<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.row_iter.next() {
            Some(&DataType::String(ref value)) => visitor.visit_str(&value),
            _ => return Err(serde::de::Error::custom("expected string cell")),
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.row_iter.next() {
            Some(&DataType::String(ref value)) => visitor.visit_string(value.clone()),
            Some(&DataType::Error(_)) => {
                return Err(serde::de::Error::custom("expected string cell; got error"))
            }
            Some(&DataType::Bool(_)) => {
                return Err(serde::de::Error::custom("expected string cell; got bool"))
            }
            Some(&DataType::Int(value)) => visitor.visit_string(format!("{}", value)),
            Some(&DataType::Float(value)) => visitor.visit_string(format!("{}", value)),
            Some(&DataType::Empty) => {
                return Err(serde::de::Error::custom("expected string cell; got empty"))
            }
            None => return Err(serde::de::Error::custom("expected string cell; got None")),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.row_iter.peek() {
            Some(DataType::Empty) | None => visitor.visit_none(),
            Some(_) => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}
impl<'a, 'de> MapAccess<'de> for RowDeserializer<'a, 'de> {
    type Error = serde::de::value::Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let key = match self.header_iter.next() {
            None | Some(None) => return Ok(None),
            Some(&Some(ref key)) => key,
        };
        seed.deserialize(BorrowedStrDeserializer::new(key))
            .map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        seed.deserialize(self)
    }
}
