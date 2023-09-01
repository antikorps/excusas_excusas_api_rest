use serde::Deserialize;
use serde::Serialize;

// INICIO BASE DATOS
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseDatos {
    pub categorias: Vec<Categoria>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categoria {
    pub nombre: String,
    pub descripcion: String,
    pub situaciones: Vec<Situaciones>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Situaciones {
    pub nombre: String,
    pub excusas: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectCategorias {
    pub nombre: String,
    pub descripcion: String, 
}
// FIN BASE DATOS

#[derive(Serialize, Debug)] 
pub struct ApiCategoria {
    pub id: i64,
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Serialize)]
pub struct ApiSituacion {
    pub id: i64,
    pub nombre: String,
}

#[derive(Serialize)]
pub struct ApiExcusa {
    pub id: i64,
    pub nombre: String,
}