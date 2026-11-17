//! Implementación del TaskRepository usando almacenamiento en memoria.
//! SUPERFICIE DE PRACTICA: Implementa los métodos del trait TaskRepository.

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::task::Task;
use crate::domain::task::TaskError;
use crate::domain::repository::TaskRepository;

#[derive(Debug, Default)]
pub struct InMemoryTaskRepository {
    tasks: Arc<Mutex<Vec<Task>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        InMemoryTaskRepository {
            tasks: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn save(&mut self, _task: Task) -> Result<Task, TaskError> {
        // TODO(reto): Implementa la lógica para guardar una tarea
        unimplemented!()
    }

    async fn find_all(&self) -> Result<Vec<Task>, TaskError> {
        // TODO(reto): Implementa la lógica para obtener todas las tareas
        unimplemented!()
    }

    async fn find_by_id(&self, _id: u64) -> Result<Option<Task>, TaskError> {
        // TODO(reto): Implementa la lógica para obtener una tarea por ID
        unimplemented!()
    }

    async fn update(&mut self, _task: Task) -> Result<Task, TaskError> {
        // TODO(reto): Implementa la lógica para actualizar una tarea
        unimplemented!()
    }
}