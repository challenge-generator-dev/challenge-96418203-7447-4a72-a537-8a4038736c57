//! Trait TaskRepository que define las operaciones de persistencia para Task.
//! SUPERFICIE DE PRACTICA: Implementa los métodos según el contrato.

use async_trait::async_trait;
use crate::domain::task::Task;
use crate::domain::task::TaskError;

#[async_trait]
pub trait TaskRepository {
    /// Guarda una tarea en el repositorio.
    async fn save(&mut self, task: Task) -> Result<Task, TaskError>;

    /// Obtiene todas las tareas del repositorio.
    async fn find_all(&self) -> Result<Vec<Task>, TaskError>;

    /// Obtiene una tarea por su ID.
    async fn find_by_id(&self, id: u64) -> Result<Option<Task>, TaskError>;

    /// Actualiza una tarea existente.
    async fn update(&mut self, task: Task) -> Result<Task, TaskError>;
}