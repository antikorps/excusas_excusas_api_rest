// INICIO CREAR

pub const CREAR_TABLAS:&str = r#"
    CREATE TABLE IF NOT EXISTS Categorias (
        id INTEGER PRIMARY KEY,
        nombre TEXT NOT NULL,
        descripcion TEXT
    );

    CREATE TABLE IF NOT EXISTS Situaciones (
        id INTEGER PRIMARY KEY,
        nombre TEXT NOT NULL,
        categoria_id INTEGER NOT NULL,
        FOREIGN KEY (categoria_id) REFERENCES Categorias (id)
    );

    CREATE TABLE IF NOT EXISTS Excusas (
        id INTEGER PRIMARY KEY,
        texto TEXT NOT NULL,
        situacion_id INTEGER NOT NULL,
        FOREIGN KEY (situacion_id) REFERENCES Situaciones (id)
    );
"#;

pub const INSERTAR_CATEGORIA:&str = "INSERT INTO Categorias (nombre, descripcion) VALUES (?, ?)";
pub const INSERTAR_SITUACION:&str = "INSERT INTO Situaciones (nombre, categoria_id) VALUES (?, ?)";
pub const INSERTAR_EXCUSA:&str = "INSERT INTO Excusas (texto, situacion_id) VALUES (?, ?)";
// FIN CREAR

pub const SELECT_CATEGORIAS:&str= "SELECT id, nombre, descripcion FROM Categorias;";
pub const SELECT_SITUACIONES:&str= "SELECT id, nombre FROM Situaciones WHERE categoria_id = ?";
pub const SELECT_EXCUSAS:&str= "SELECT id, texto FROM Excusas WHERE situacion_id = ?";