//! Handlers de aplicación que orquestan la lógica de negocio para las operaciones sobre tareas.
//! SUPERFICIE DE PRACTICA: Implementa los handlers utilizando el TaskRepository.

use actix_web::{web, HttpResponse, Result};
use crate::domain::task::Task;
use crate::domain::repository::TaskRepository;
use std::sync::Arc;

/// Crea una nueva tarea.
pub async fn create_task<T: TaskRepository>(
    task: web::Json<Task>,
    repo: web::Data<Arc<tokio::sync::Mutex<T>>>,
) -> Result<HttpResponse> {
    // TODO(reto): Implementa la lógica para crear una tarea
    let _ = task;
    let _ = repo;
    Ok(HttpResponse::Ok().json("create_task not implemented"))
}

/// Lista todas las tareas.
pub async fn list_tasks<T: TaskRepository>(
    repo: web::Data<Arc<tokio::sync::Mutex<T>>>,
) -> Result<HttpResponse> {
    // TODO(reto): Implementa la lógica para listar tareas
    let _ = repo;
    Ok(HttpResponse::Ok().json("list_tasks not implemented"))
}

/// Actualiza una tarea existente.
pub async fn update_task<T: TaskRepository>(
    id: web::Path<u64>,
    task: web::Json<Task>,
    repo: web::Data<Arc<tokio::sync::Mutex<T>>>,
) -> Result<HttpResponse> {
    // TODO(reto): Implementa la lógica para actualizar una tarea
    let _ = id;
    let _ = task;
    let _ = repo;
    Ok(HttpResponse::Ok().json("update_task not implemented"))
}