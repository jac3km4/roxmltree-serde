# roxmltree-serde
Serde XML Deserializer based on [roxmltree](https://github.com/RazrFalcon/roxmltree) with support for zero-copy parsing.

## usage
```rs
#[derive(Debug, Deserialize)]
pub struct Channel<'a> {
    // regular field, accepts either an attribute (title="Tech News")
    // or a child text node (<title>Tech News</title>)
    pub title: &'a str,
    // accepts multiple child nodes with tag <item>
    #[serde(rename = "item", default)]
    pub items: Vec<Item<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct Item<'a> {
    // optional attribute or child
    pub guid: Option<Guid<'a>>,
    // accepts multiple child nodes with tag 'content' prefixed by a namespace
    #[serde(rename = "$ns:content", default)]
    pub media: Vec<MediaContent<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct Guid<'a> {
    // parses the contents of node as text (<guid>this</guid>)
    #[serde(rename = "$text")]
    pub value: &'a str,
}
```

Complete examples can be found [in the tests](src/de/mod.rs#L56-L128).
