use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Href
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Href {
    pub href: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeNode {
    #[serde(rename = "type")]
    pub type_value: String,
    pub name: String,
}