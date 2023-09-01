use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Row};

use crate::modelos;
use crate::consultas;

#[get("/categorias")]
pub async fn categorias(pool: web::Data<SqlitePool>) -> impl Responder {

    let filas: Vec<SqliteRow>;

    match sqlx::query(consultas::SELECT_CATEGORIAS)
        .fetch_all(&**pool).await {
        Ok(ok) => {
            filas = ok;
        }
        Err(error) => {
            eprintln!("no se ha podido procesar la petición de categorias: {error}");
            return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body("{\"info:\": \"Esto no debía haber pasado. No tardaremos en corregirlo\"}");
        }
    };

    let mut categorias = Vec::new();
    for fila in filas {
        let id: i64 = fila.get("id");
        let nombre: String = fila.get("nombre");
        let descripcion: String = fila.get("descripcion");
        categorias.push(modelos::ApiCategoria {
            id,
            nombre,
            descripcion,
        });
    }

    if categorias.is_empty() {
        eprintln!("las categorías nunca pueden quedar vacías");
        return HttpResponse::InternalServerError()
        .content_type("application/json")
        .body("{\"info:\": \"Esto no debía haber pasado. No tardaremos en corregirlo\"}");
    }

    let cuerpo: String;
    match serde_json::to_string(&categorias){
        Ok(ok) => {
            cuerpo = ok;
        }
        Err(error) => {
            eprintln!("no se ha podido parsear ApiCategorias a json: {error}");
            return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body("{\"info:\": \"Esto no debía haber pasado. No tardaremos en corregirlo\"}");
        }
    }

    HttpResponse::Ok()
    .content_type("application/json")
    .body(cuerpo)
}