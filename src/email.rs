use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Email {
    pub host: String,
    pub body: String,
}
