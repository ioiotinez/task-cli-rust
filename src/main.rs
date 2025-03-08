use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    description: String,
    done: bool,
}

fn charge_tasks() -> Vec<Task> {
    // creo un path para el archivo tasks.json
    let path = Path::new("tasks.json");
    // si el archivo no existe, devuelvo un vector vacio
    if !path.exists() {
        return Vec::new();
    }
    // leo el archivo y lo convierto a un string
    let data = fs::read_to_string(path).expect("Unable to read file");
    // convierto el string a un vector de tareas
    serde_json::from_str(&data).expect("Unable to parse json")
}

fn save_tasks(tasks: &Vec<Task>) {
    // convierto el vector de tareas a un string json
    let data = serde_json::to_string(&tasks).expect("Unable to serialize tasks");
    // creo un archivo tasks.json y escribo el string json
    let mut file = File::create("tasks.json").expect("Unable to create file");
    // escribo el string en el archivo
    file.write_all(data.as_bytes()).expect("Unable to write file");
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    // creo un id para la tarea
    let id = tasks.len() as i32 + 1;
    // creo la tarea
    let task = Task {
        id,
        description,
        done: false,
    };
    // agrego la tarea al vector
    tasks.push(task);
    // guardo el vector en el archivo
    save_tasks(tasks);
    // muestro un mensaje
    println!("Task added");
}

fn list_tasks(tasks: &Vec<Task>) {
    // si el vector esta vacio, muestro un mensaje
    if tasks.is_empty() {
        println!("No tasks.");
        return;
    }

    // recorro el vector y muestro las tareas
    for task in tasks {
        // si la tarea esta completada, muestro "Completed", sino "Pending"
        let done = if task.done { "Completed"} else { "Pending" };
        // muestro la tarea
        println!("{} - {} - {}", task.id, task.description, done);
    }
}

fn change_task_status(tasks: &mut Vec<Task>, id: i32) {
    // busco la tarea por id y cambio el estado
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        // cambio el estado de la tarea
        task.done = !task.done;
        // guardo el vector en el archivo
        save_tasks(tasks);
        // muestro un mensaje
        println!("Task status changed");
    } else {
        // si no se encontro la tarea, muestro un mensaje
        println!("Task not found");
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: i32) {
   // busco la tarea por id y la elimino
   // si se encontro la tarea, se elimina y se guarda el vector en el archivo
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        // elimino la tarea del vector
        tasks.remove(index);
        save_tasks(tasks);
        println!("Task {} deleted", id);
    } else {
        println!("Task not found");
    }
}

fn main() {
    // cargo las tareas del archivo
    let mut tasks = charge_tasks();
    // muestro el menu
    loop {
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Change task status");
        println!("4. Delete task");
        println!("5. Exit");
        println!("Choose an option:");

        // leo la opcion ingresada por el usuario
        let mut option = String::new();
        // leo la opcion ingresada por el usuario
        std::io::stdin().read_line(&mut option).expect("Failed to read line");

        match option.trim() {
            "1" => {
                println!("Enter the task description:");
                let mut description = String::new();
                std::io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                add_task(&mut tasks, description.trim().to_string());
            }
            "2" => list_tasks(&tasks),
            "3" => {
                println!("Enter the task id:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<i32>().expect("Invalid id");
                change_task_status(&mut tasks, id);
            }
            "4" => {
                println!("Enter the task id:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<i32>().expect("Invalid id");
                delete_task(&mut tasks, id);
            }
            "5" => break,
            _ => println!("Invalid option"),
        }
    }
}