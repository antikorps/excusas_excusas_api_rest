use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Row};

use crate::modelos;
use crate::consultas;

#[get("/categorias/{categoria}/situaciones")]
pub async fn situaciones(pool: web::Data<SqlitePool>, categoria: web::Path<String>) -> impl Responder {
    let categoria_id: u8;
    match categoria.parse::<u8>() {
        Err(_) => {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .body("{\"info:\": \"La url introducida no es válida\"}");
        }
        Ok(ok) => {
            categoria_id = ok;
        }
    }

    let filas: Vec<SqliteRow>;

    match sqlx::query(consultas::SELECT_SITUACIONES)
    .bind(categoria_id.clone())
    .fetch_all(&**pool).await {
        Ok(ok) => {
            filas = ok;
        }
        Err(error) => {
            eprintln!("no se ha podido procesar la petición de situaciones: {error}");
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{\"info:\": \"Esto no debía haber pasado. No tardaremos en corregirlo\"}");
        }
    };

    let mut situaciones = Vec::new();
    for fila in filas {
        let id: i64 = fila.get("id");
        let nombre: String = fila.get("nombre");
        
        situaciones.push(modelos::ApiSituacion {
            id,
            nombre,
        });
    }

    if situaciones.is_empty() {
        return HttpResponse::BadRequest()
                .content_type("application/json")
                .body("{\"info:\": \"La url introducida no es válida\"}");
    }

    let cuerpo: String;
    match serde_json::to_string(&situaciones){
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