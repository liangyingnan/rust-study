use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MyObject {
    pub id: u32,
    pub name: String,
}


