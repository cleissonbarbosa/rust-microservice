use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Usuario {
    nome: String,
    email: String,
}

#[get("/")]
async fn index() -> impl Responder {
    "Bem-vindo ao meu microserviço em Rust!"
}

#[post("/usuarios")]
async fn criar_usuario(_usuario: web::Json<Usuario>) -> HttpResponse {
    // Lógica para criar um usuário
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run();

    println!("Servidor está online em http://127.0.0.1:8080");

    server.await
}
