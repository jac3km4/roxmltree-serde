use serde::de;

use crate::error::{Error, Result};
use crate::NodeKind;

macro_rules! parse_impl {
    ($ty:ty, $fn_name:ident, $visit:ident) => {
        fn $fn_name<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            let res = self
                .text
                .parse::<$ty>()
                .map_err(|err| Error::ParseError(err.to_string()))?;
            visitor.$visit(res)
        }
    };
}

pub struct TextDeserializer<'de> {
    text: &'de str,
}

impl<'de> TextDeserializer<'de> {
    #[inline]
    pub(crate) fn new(attr: &'de str) -> Self {
        Self { text: attr }
    }
}

impl<'de> de::Deserializer<'de> for TextDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_borrowed_str(self.text)
    }

    parse_impl!(bool, deserialize_bool, visit_bool);
    parse_impl!(i8, deserialize_i8, visit_i8);
    parse_impl!(i16, deserialize_i16, visit_i16);
    parse_impl!(i32, deserialize_i32, visit_i32);
    parse_impl!(i64, deserialize_i64, visit_i64);
    parse_impl!(u8, deserialize_u8, visit_u8);
    parse_impl!(u16, deserialize_u16, visit_u16);
    parse_impl!(u32, deserialize_u32, visit_u32);
    parse_impl!(u64, deserialize_u64, visit_u64);
    parse_impl!(f32, deserialize_f32, visit_f32);
    parse_impl!(f64, deserialize_f64, visit_f64);
    parse_impl!(char, deserialize_char, visit_char);

    #[inline]
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_borrowed_str(self.text)
    }

    #[inline]
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_string(self.text.to_owned())
    }

    #[inline]
    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_borrowed_bytes(self.text.as_bytes())
    }

    #[inline]
    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_byte_buf(self.text.as_bytes().to_vec())
    }

    #[inline]
    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.text.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.text.is_empty() {
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedUnit(NodeKind::Text))
        }
    }

    #[inline]
    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(Error::ExpectedArray(NodeKind::Text))
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(Error::ExpectedArray(NodeKind::Text))
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(Error::ExpectedArray(NodeKind::Text))
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(Error::ExpectedMap(NodeKind::Text))
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(Error::ExpectedMap(NodeKind::Text))
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(Error::ExpectedEnum(NodeKind::Text))
    }

    #[inline]
    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
}
