use poem::{error::BadRequest, Result, Route, Server};
use poem_openapi::{
    payload::{Json},
    types::multipart::Upload,
    ApiResponse, Multipart, Object, OpenApi, OpenApiService,
};

use uuid::Uuid;
use crate::api::files::Files;
use std::{ fs::{File, self}, io::Write };

impl Files {

}

pub struct FileApi;

#[derive(Debug, Multipart)]
struct UploadPayload {
    name: String,
    desc: Option<String>,
    file: Upload,
}

#[OpenApi]
impl FileApi {
    /// Upload file
    #[oai(path = "/uploadFiles", method = "post")]
    async fn upload(&self, upload: UploadPayload) -> Result<Json<u64>> {
        let file = Files {
            name: upload.name,
            desc: upload.desc,
            content_type: upload.file.content_type().map(ToString::to_string),
            filename: upload.file.file_name().map(ToString::to_string),
            data: upload.file.into_vec().await.map_err(BadRequest)?,
        };

        let id = Uuid::new_v4().to_string().replacen("-", "", 4);
        if let Some(file_name) = file.filename {
            let file_ext = file_name.split(".").last().unwrap();
            let path_str = format!("./tmp_files/{}.{}", id, file_ext);
            let mut file_ = File::create(path_str).unwrap();
            file_.write_all(&file.data).unwrap();
        }

        Ok(Json(1233242332))
    }
}