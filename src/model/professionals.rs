use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Professional {
    pub id: Option<i32>,
    pub name: String,
    pub title: String,
    pub department: String,
}
