use actix_web::{web, App, HttpServer};
mod bbdd;
mod consultas;
mod modelos;

mod res_categorias;
mod res_situaciones;
mod res_excusas;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = bbdd::json_a_sqlite().await.expect("no ha podido gestionarse la creación de la base de datos y el pool de conexiones:");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) 
            .service(res_categorias::categorias)
            .service(res_situaciones::situaciones)
            .service(res_excusas::excusas)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}