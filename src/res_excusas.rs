use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Row};

use crate::modelos;
use crate::consultas;

#[get("/situaciones/{situacion}/excusas")]
pub async fn excusas(pool: web::Data<SqlitePool>, situacion: web::Path<String>) -> impl Responder {
    let situacion_id: u8;
    match situacion.parse::<u8>() {
        Err(_) => {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .body("{\"info:\": \"La url introducida no es válida\"}");
        }
        Ok(ok) => {
            situacion_id = ok;
        }
    }

    let filas: Vec<SqliteRow>;

    match sqlx::query(consultas::SELECT_EXCUSAS)
    .bind(situacion_id.clone())
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

    let mut excusas = Vec::new();
    for fila in filas {
        let id: i64 = fila.get("id");
        let nombre: String = fila.get("texto");
        
        excusas.push(modelos::ApiExcusa {
            id,
            nombre,
        });
    }

    if excusas.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body("{\"info:\": \"La url introducida no es válida\"}");
    }

    let cuerpo: String;
    match serde_json::to_string(&excusas){
        Ok(ok) => {
            cuerpo = ok;
        }
        Err(error) => {
            eprintln!("no se ha podido parsear ApiExcusas a json: {error}");
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{\"info:\": \"Esto no debía haber pasado. No tardaremos en corregirlo\"}");
        }
    }

    HttpResponse::Ok()
    .content_type("application/json")
    .body(cuerpo)
}
