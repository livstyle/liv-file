use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

use std::io::Error;

use crate::cores::{db};
// use crate::api::{apic::{init}};


struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
pub async fn bootstrap() -> Result<(), Error> {

    // TODO 初始化数据库 不使用时可以去掉
    // db::init_db_pool().await;
        
    // 初始化Web服务
    let api_service =
    OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/swagger", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await

}