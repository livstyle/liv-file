pub mod route;

use poem_openapi::{ Object};

#[derive(Debug, Clone, Object)]
pub struct Files {
    name: String,
    desc: Option<String>,
    content_type: Option<String>,
    filename: Option<String>,
    data: Vec<u8>,
}

