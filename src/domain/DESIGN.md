//! Diseño del componente de gestión de tareas
//!
//! ## Arquitectura
//!
//! Este componente sigue una arquitectura en capas:
//! - **Dominio**: Contiene las entidades (`Task`) y los contratos (`TaskRepository`).
//! - **Aplicación**: Contiene los handlers que orquestan la lógica de negocio.
//! - **Infraestructura**: Implementa los contratos definidos en el dominio (ej: `InMemoryTaskRepository`).
//!
//! ## Responsabilidades por capa
//!
//! ### Dominio
//! - `Task`: Entidad que representa una tarea con sus atributos y validaciones.
//! - `TaskRepository`: Trait que define las operaciones de persistencia.
//!
//! ### Aplicación
//! - `handlers.rs`: Contiene los handlers que exponen la lógica de negocio a través de Actix Web.
//!   - `create_task`: Crea una nueva tarea.
//!   - `list_tasks`: Lista todas las tareas.
//!   - `update_task`: Actualiza una tarea existente.
//!
//! ### Infraestructura
//! - `InMemoryTaskRepository`: Implementación en memoria del `TaskRepository`.
//!
//! ## Flujo de datos
//!
//! 1. **Creación de tarea**:
//!    - El handler `create_task` recibe una solicitud HTTP con los datos de la tarea.
//!    - Valida los datos y delega la persistencia al `TaskRepository`.
//!    - Retorna la tarea creada con su ID asignado.
//!
//! 2. **Listado de tareas**:
//!    - El handler `list_tasks` solicita todas las tareas al `TaskRepository`.
//!    - Retorna la lista de tareas en formato JSON.
//!
//! 3. **Actualización de tarea**:
//!    - El handler `update_task` recibe el ID de la tarea y los nuevos datos.
//!    - Valida los datos y delega la actualización al `TaskRepository`.
//!    - Retorna la tarea actualizada.
//!
//! ## Manejo de errores
//!
//! Los errores de dominio se definen en el enum `TaskError` y se mapean a respuestas HTTP utilizando
//! la implementación de `actix_web::error::ResponseError` para `TaskError`.
//!
//! ## Consideraciones técnicas
//!
//! - **Concurrencia**: El repositorio en memoria utiliza `Arc<Mutex<Vec<Task>>>` para garantizar seguridad en hilos.
//! - **Inyección de dependencias**: Los handlers reciben el `TaskRepository` como `web::Data<Arc<Mutex<T>>>`.
//! - **Serialización**: Las entidades implementan `Serialize` y `Deserialize` de `serde` para facilitar el intercambio de datos.