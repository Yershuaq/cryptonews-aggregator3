use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use tera::{Tera, Context};

mod api;
mod handlers;
mod models;
mod utils;

// 👇 Главная HTML-страница с формой поиска
async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("🚀 Сервер запущен: http://127.0.0.1:8080");

    // Инициализация шаблонов Tera
    let tera = Tera::new("src/templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/search", web::post().to(handlers::news_handler::search))
            .route("/news/{symbol}", web::get().to(handlers::news_handler::get_news))
            // 🔥 Подключение CSS-файлов из /static
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
