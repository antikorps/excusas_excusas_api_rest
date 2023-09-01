use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::error::Error;

use crate::modelos;
use crate::consultas;

pub async fn json_a_sqlite() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let archivo_json = include_str!("bbdd.json");
    let bbdd: modelos::BaseDatos = serde_json::from_str(archivo_json)?;

    let pool = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::query(consultas::CREAR_TABLAS).execute(&pool).await?;
    
    for categoria in bbdd.categorias {

        let insertar_categoria = sqlx::query(consultas::INSERTAR_CATEGORIA)
            .bind(categoria.nombre)
            .bind(categoria.descripcion)
            .execute(&pool).await?;
        let categoria_id = insertar_categoria.last_insert_rowid();

        for situacion in categoria.situaciones {
            let insertar_situacion = sqlx::query(consultas::INSERTAR_SITUACION)
                .bind(situacion.nombre)
                .bind(categoria_id)
                .execute(&pool).await?;

            let situacion_id = insertar_situacion.last_insert_rowid();

            for excusa in situacion.excusas {
                sqlx::query(consultas::INSERTAR_EXCUSA)
                    .bind(excusa)
                    .bind(situacion_id)
                    .execute(&pool).await?;
            }
        }
    }
    Ok(pool)
}