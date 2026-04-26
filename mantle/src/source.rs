use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub enum FileSource {
    File(String),
    Embedded(String),
    Url(String),
}