use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "mini-backend")]
#[command(about = "CLI backend demo", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Agrega una nueva tarea a la lista
    Add {
        #[arg(short, long)]
        title: String,
    },
    /// Lista todas las tareas
    List,
    /// Marca una tarea como completada por su ID
    Complete {
        #[arg(short, long)]
        id: u32,
    },
}

const DB_FILE: &str = "db.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { title } => {
            let mut tasks = load_tasks()?;
            // Generar ID autoincremental simple
            let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            let task = Task {
                id: new_id,
                title: title.clone(),
                completed: false,
            };

            tasks.push(task.clone());
            save_tasks(&tasks)?;
            println!("Tarea agregada: ID {} - {}", task.id, task.title);
        }
        Commands::List => {
            let tasks = load_tasks()?;
            if tasks.is_empty() {
                println!("No hay tareas registradas.");
            } else {
                println!("{:<5} | {:<10} | {}", "ID", "Estado", "Título");
                println!("{}", "-".repeat(40));
                for task in tasks {
                    let status = if task.completed { "[Hecho]" } else { "[ ]" };
                    println!("{:<5} | {:<10} | {}", task.id, status, task.title);
                }
            }
        }
        Commands::Complete { id } => {
            let mut tasks = load_tasks()?;
            // find() devuelve una referencia, iter_mut() nos permite modificar
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
                task.completed = true;
                save_tasks(&tasks)?;
                println!("Tarea {} marcada como completada.", id);
            } else {
                println!("No se encontró la tarea con ID {}.", id);
            }
        }
    }

    Ok(())
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::File::create(DB_FILE)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if !Path::new(DB_FILE).exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(DB_FILE)?;
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }
    // Intentamos deserializar. Si falla (ej. formato antiguo), devolvemos lista vacía o error
    match serde_json::from_str(&content) {
        Ok(tasks) => Ok(tasks),
        Err(_) => {
            // Si el formato no coincide (ej. era un objeto único y ahora esperamos array),
            // podríamos resetear o lanzar error. Aquí lanzamos error para que el usuario sepa.
            Err("El formato de db.json no es válido o es incompatible. Borra el archivo para reiniciar.".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_crud_operations() {
        // Limpieza previa
        let _ = fs::remove_file(DB_FILE);

        // 1. Agregar
        let task1 = Task { id: 1, title: "Test 1".to_string(), completed: false };
        let tasks = vec![task1];
        assert!(save_tasks(&tasks).is_ok());

        // 2. Leer
        let loaded = load_tasks().expect("Falló carga");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title, "Test 1");

        // Limpieza final
        let _ = fs::remove_file(DB_FILE);
    }
}
