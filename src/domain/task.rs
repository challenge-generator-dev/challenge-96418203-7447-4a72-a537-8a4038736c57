//! Entidad Task que representa una tarea en el sistema.
//! SUPERFICIE DE PRACTICA: Implementa los métodos y validaciones necesarias.

use serde::{Serialize, Deserialize};
use thiserror::Error;
use actix_web::{ResponseError, http::StatusCode};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Option<u64>,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("El título de la tarea no puede estar vacío")]
    EmptyTitle,
    #[error("La descripción de la tarea no puede estar vacía")]
    EmptyDescription,
    #[error("Error al adquirir el lock del repositorio")]
    LockError,
    #[error("Tarea no encontrada")]
    NotFound,
}

impl ResponseError for TaskError {
    fn status_code(&self) -> StatusCode {
      todo!("TODO(reto): status_code")
    }
}

impl Task {
    /// Crea una nueva tarea con los campos proporcionados.
    /// Valida que el título y la descripción no estén vacíos.
    pub fn new(title: String, description: String) -> Result<Self, TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        if description.trim().is_empty() {
            return Err(TaskError::EmptyDescription);
        }
        Ok(Task {
            id: None,
            title,
            description,
            completed: false,
        })
    }

    /// Marca la tarea como completada.
    pub fn complete(&mut self) {
      todo!("TODO(reto): complete")
    }
}