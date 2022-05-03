use serde::Deserialize;

use crate::Result;

mod children;
mod text;

pub type Deserializer<'doc, 'de> = children::SingleDeserializer<'doc, 'de>;

pub fn from_doc<'a, T>(doc: &'a roxmltree::Document) -> Result<T>
where
    T: Deserialize<'a>,
{
    T::deserialize(Deserializer::one(doc.root_element()))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use roxmltree::Document;
    use serde_derive::Deserialize;

    use super::*;

    #[test]
    fn parse_simple_xml() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Note<'a> {
            to: &'a str,
            from: &'a str,
            heading: &'a str,
            body: &'a str,
        }

        let xml = r#"
            <note>
                <to>Tove</to>
                <from>Jani</from>
                <heading>Reminder</heading>
                <body>Don't forget me this weekend!</body>
            </note>"#;

        let doc = Document::parse(xml).unwrap();
        let note: Note = from_doc(&doc).unwrap();

        assert_eq!(note, Note {
            to: "Tove",
            from: "Jani",
            heading: "Reminder",
            body: "Don't forget me this weekend!"
        })
    }

    #[test]
    fn parse_array_xml() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct DiskId(u64);

        #[derive(Deserialize, PartialEq, Debug)]
        #[serde(rename_all = "lowercase")]
        enum Genre {
            Rock,
            Metal,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct Genres {
            #[serde(rename = "$all")]
            values: Vec<Genre>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct Disk<'a> {
            id: DiskId,
            title: Cow<'a, str>,
            genres: Genres,
            price: f32,
            year: i32,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct Catalog<'a> {
            #[serde(rename = "disk")]
            disks: Vec<Disk<'a>>,
        }

        let xml = r#"
            <catalog>
                <disk id="453678">
                    <title>Empire Burlesque</title>
                    <genres><rock/></genres>
                    <price>10.90</price>
                    <year>1985</year>
                </disk>
                <disk id="845783">
                    <title>Hide your heart</title>
                    <genres><metal/></genres>
                    <price>9.90</price>
                    <year>1988</year>
                </disk>
            </catalog>"#;

        let doc = Document::parse(xml).unwrap();
        let catalog: Catalog = from_doc(&doc).unwrap();

        assert_eq!(catalog, Catalog {
            disks: vec![
                Disk {
                    id: DiskId(453678),
                    title: Cow::Borrowed("Empire Burlesque"),
                    genres: Genres {
                        values: vec![Genre::Rock]
                    },
                    price: 10.90,
                    year: 1985
                },
                Disk {
                    id: DiskId(845783),
                    title: Cow::Borrowed("Hide your heart"),
                    genres: Genres {
                        values: vec![Genre::Metal]
                    },
                    price: 9.90,
                    year: 1988
                },
            ]
        })
    }
}
