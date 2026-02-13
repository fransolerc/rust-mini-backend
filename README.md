# Rust Mini Backend CLI

An introductory project to learn Rust fundamentals, simulating a simple backend via a Command Line Interface (CLI).

## 🚀 Features

This project implements a basic Task Management system (To-Do List) with JSON file persistence.

*   **Robust CLI**: Arguments and subcommands handled with `clap`.
*   **Persistence**: Read/Write data in JSON format using `serde` and `serde_json`.
*   **Basic CRUD**:
    *   Create tasks (`add`)
    *   List tasks (`list`)
    *   Update status (`complete`)
*   **Error Handling**: Idiomatic use of `Result` and error propagation.

## 🛠️ Requirements

*   [Rust & Cargo](https://www.rust-lang.org/tools/install) (v1.70+)

## 📦 Installation & Usage

1.  **Clone the repository**:
    ```bash
    git clone <repo-url>
    cd rust-mini-backend
    ```

2.  **Build the project**:
    ```bash
    cargo build
    ```

3.  **Run commands**:

    *   **Add a task**:
        ```bash
        cargo run -- add --title "Learn Rust"
        ```

    *   **List all tasks**:
        ```bash
        cargo run -- list
        ```
        *Output:*
        ```text
        ID    | Status     | Title
        ----------------------------------------
        1     | [ ]        | Learn Rust
        ```

    *   **Complete a task**:
        ```bash
        cargo run -- complete --id 1
        ```

    *   **View help**:
        ```bash
        cargo run -- --help
        ```

## 🧪 Tests

To run the integrated unit tests:

```bash
cargo test
```

## 📚 Project Structure

*   `src/main.rs`: Main source code. Contains the `Task` struct definition, CLI logic, and file operations.
*   `Cargo.toml`: Dependency manager (equivalent to `pom.xml` or `build.gradle`).
*   `db.json`: File where data is stored (created automatically).

## 📝 Learning Notes (Java vs Rust)

*   **Objects**: `struct` instead of `class`.
*   **Serialization**: `#[derive(Serialize, Deserialize)]` (Serde) instead of Jackson/Gson.
*   **Errors**: Explicit `Result<T, E>` instead of `try-catch` exceptions.
*   **Null**: `Option<T>` instead of `null`.
