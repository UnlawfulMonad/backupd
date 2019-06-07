use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FileHeader {
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
    last: bool,
    data: Vec<u8>,
}
