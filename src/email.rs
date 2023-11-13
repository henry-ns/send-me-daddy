use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Email {
    pub receiver: String,
    pub subject: String,
    pub body: String,
}
