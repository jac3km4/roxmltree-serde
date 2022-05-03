use std::iter;

use serde::{de, Deserializer};

use super::text::TextDeserializer;
use crate::error::{Error, Result};
use crate::NodeKind;

macro_rules! parse_impl {
    ($fn_name:ident) => {
        #[inline]
        fn $fn_name<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            self.into_text_deser()
                .ok_or(Error::NodesExhausted)?
                .$fn_name(visitor)
        }
    };
}

pub type SingleDeserializer<'doc, 'de> =
    ChildrenDeserializer<'doc, 'de, iter::Once<roxmltree::Node<'doc, 'de>>>;

pub struct ChildrenDeserializer<'doc, 'de: 'doc, I>
where
    I: Iterator<Item = roxmltree::Node<'doc, 'de>>,
{
    children: I,
}

impl<'doc: 'de, 'de, I> ChildrenDeserializer<'doc, 'de, I>
where
    I: Iterator<Item = roxmltree::Node<'doc, 'de>>,
{
    #[inline]
    fn new(children: I) -> Self {
        Self { children }
    }

    #[inline]
    fn into_child(mut self) -> Option<roxmltree::Node<'doc, 'de>> {
        self.children.next()
    }

    #[inline]
    fn into_text(self) -> Option<&'de str> {
        self.into_child().and_then(|n| n.text())
    }

    #[inline]
    fn into_text_deser(self) -> Option<TextDeserializer<'de>> {
        self.into_text().map(TextDeserializer::new)
    }
}

impl<'doc: 'de, 'de> ChildrenDeserializer<'doc, 'de, iter::Once<roxmltree::Node<'doc, 'de>>> {
    #[inline]
    pub fn one(one: roxmltree::Node<'doc, 'de>) -> Self {
        Self::new(iter::once(one))
    }
}

impl<'doc: 'de, 'de, I> de::Deserializer<'de> for ChildrenDeserializer<'doc, 'de, I>
where
    I: Iterator<Item = roxmltree::Node<'doc, 'de>>,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    parse_impl!(deserialize_bool);
    parse_impl!(deserialize_i8);
    parse_impl!(deserialize_i16);
    parse_impl!(deserialize_i32);
    parse_impl!(deserialize_i64);
    parse_impl!(deserialize_u8);
    parse_impl!(deserialize_u16);
    parse_impl!(deserialize_u32);
    parse_impl!(deserialize_u64);
    parse_impl!(deserialize_f32);
    parse_impl!(deserialize_f64);
    parse_impl!(deserialize_char);

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let str = self.into_text().unwrap_or_default();
        visitor.visit_borrowed_str(str)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let str = self.into_text().unwrap_or_default();
        visitor.visit_string(str.to_owned())
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let str = self.into_text().unwrap_or_default();
        visitor.visit_borrowed_bytes(str.as_bytes())
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let str = self.into_text().unwrap_or_default();
        visitor.visit_byte_buf(str.as_bytes().to_vec())
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(child) = self.into_child() {
            visitor.visit_some(ChildrenDeserializer::one(child))
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.into_child().is_some() {
            Err(Error::ExpectedUnit(NodeKind::Elem))
        } else {
            visitor.visit_unit()
        }
    }

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

    #[inline]
    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(self)
    }

    #[inline]
    fn deserialize_tuple<V: de::Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let node = self.into_child().ok_or(Error::NodesExhausted)?;
        visitor.visit_map(NodeMapAcces::new(fields, node))
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let node = self.into_child().ok_or(Error::NodesExhausted)?;
        visitor.visit_enum(NodeEnumAccess::new(node))
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut peek = self.children.peekable();
        let node = peek.peek().ok_or(Error::NodesExhausted)?;
        visitor.visit_borrowed_str(node.tag_name().name())
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
}

impl<'doc: 'de, 'de, I> de::SeqAccess<'de> for ChildrenDeserializer<'doc, 'de, I>
where
    I: Iterator<Item = roxmltree::Node<'doc, 'de>>,
{
    type Error = Error;

    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.children
            .next()
            .map(|node| seed.deserialize(ChildrenDeserializer::one(node)))
            .transpose()
    }
}

struct NodeMapAcces<'doc, 'de> {
    fields: &'static [&'static str],
    node: roxmltree::Node<'doc, 'de>,
}

impl<'doc, 'de> NodeMapAcces<'doc, 'de> {
    #[inline]
    fn new(fields: &'static [&'static str], node: roxmltree::Node<'doc, 'de>) -> Self {
        Self { fields, node }
    }
}

impl<'doc: 'de, 'de> de::MapAccess<'de> for NodeMapAcces<'doc, 'de> {
    type Error = Error;

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.fields
            .first()
            .map(|str| seed.deserialize(TextDeserializer::new(*str)))
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.fields.split_first() {
            Some((head, tail)) => {
                self.fields = tail;
                if *head == "$text" {
                    let txt = self.node.text().ok_or(Error::ExpectedText(NodeKind::Elem))?;
                    seed.deserialize(TextDeserializer::new(txt))
                } else if *head == "$all" {
                    seed.deserialize(ChildrenDeserializer::new(self.node.children()))
                } else if let Some(tag) = head.strip_prefix("$ns:") {
                    let it = self
                        .node
                        .children()
                        .filter(|node| node.has_tag_name(tag) && node.tag_name().namespace().is_some());
                    seed.deserialize(ChildrenDeserializer::new(it))
                } else if let Some(attr) = self.node.attribute(*head) {
                    seed.deserialize(TextDeserializer::new(attr))
                } else {
                    let it = self.node.children().filter(|node| node.has_tag_name(*head));
                    seed.deserialize(ChildrenDeserializer::new(it))
                }
            }
            None => Err(Error::NodesExhausted),
        }
    }
}

struct NodeEnumAccess<'doc, 'de> {
    node: roxmltree::Node<'doc, 'de>,
}

impl<'doc, 'de> NodeEnumAccess<'doc, 'de> {
    #[inline]
    fn new(node: roxmltree::Node<'doc, 'de>) -> Self {
        Self { node }
    }
}

impl<'doc: 'de, 'de> de::EnumAccess<'de> for NodeEnumAccess<'doc, 'de> {
    type Error = Error;

    type Variant = Self;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let res = seed.deserialize(ChildrenDeserializer::one(self.node))?;
        Ok((res, self))
    }
}

impl<'doc: 'de, 'de> de::VariantAccess<'de> for NodeEnumAccess<'doc, 'de> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(ChildrenDeserializer::one(self.node))
    }

    #[inline]
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ChildrenDeserializer::new(self.node.children()).deserialize_seq(visitor)
    }

    #[inline]
    fn struct_variant<V: de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_map(NodeMapAcces::new(fields, self.node))
    }
}
