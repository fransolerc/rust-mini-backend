# Rust Mini Backend CLI

Un proyecto introductorio para aprender los fundamentos de Rust, simulando un backend simple mediante una herramienta de línea de comandos (CLI).

## 🚀 Características

Este proyecto implementa un sistema básico de gestión de tareas (To-Do List) con persistencia en archivo JSON.

*   **CLI Robusto**: Argumentos y subcomandos manejados con `clap`.
*   **Persistencia**: Lectura y escritura de datos en formato JSON usando `serde` y `serde_json`.
*   **CRUD Básico**:
    *   Crear tareas (`add`)
    *   Listar tareas (`list`)
    *   Actualizar estado (`complete`)
*   **Manejo de Errores**: Uso idiomático de `Result` y propagación de errores.

## 🛠️ Requisitos

*   [Rust & Cargo](https://www.rust-lang.org/tools/install) (v1.70+)

## 📦 Instalación y Uso

1.  **Clonar el repositorio** (o descargar el código):
    ```bash
    git clone <url-del-repo>
    cd rust-mini-backend
    ```

2.  **Compilar el proyecto**:
    ```bash
    cargo build
    ```

3.  **Ejecutar comandos**:

    *   **Agregar una tarea**:
        ```bash
        cargo run -- add --title "Aprender Rust"
        ```

    *   **Listar todas las tareas**:
        ```bash
        cargo run -- list
        ```
        *Salida:*
        ```text
        ID    | Estado     | Título
        ----------------------------------------
        1     | [ ]        | Aprender Rust
        ```

    *   **Completar una tarea**:
        ```bash
        cargo run -- complete --id 1
        ```

    *   **Ver ayuda**:
        ```bash
        cargo run -- --help
        ```

## 🧪 Tests

Para ejecutar las pruebas unitarias integradas:

```bash
cargo test
```

## 📚 Estructura del Proyecto

*   `src/main.rs`: Código fuente principal. Contiene la definición de la estructura `Task`, la lógica del CLI y las operaciones de archivo.
*   `Cargo.toml`: Gestor de dependencias (equivalente a `pom.xml` o `build.gradle`).
*   `db.json`: Archivo donde se almacenan los datos (se crea automáticamente).

## 📝 Notas de Aprendizaje (Java vs Rust)

*   **Objetos**: `struct` en lugar de `class`.
*   **Serialización**: `#[derive(Serialize, Deserialize)]` (Serde) en lugar de Jackson/Gson.
*   **Errores**: `Result<T, E>` explícito en lugar de `try-catch` exceptions.
*   **Null**: `Option<T>` en lugar de `null`.
