# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener el proyecto completo y arrancable. Guarda los archivos que te devuelva en la
carpeta del repo, respetando exactamente las rutas.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Superficie de practica — NO resuelvas

Estos archivos SON el ejercicio de la persona. No los implementes; deja stubs.

- `src/domain/task.rs` — La fase "Configuración inicial y creación de tareas" (f1) practica Task struct (definición con #[derive(Serialize, Deserialize)]) aqui: este archivo es el ejercicio (blueprint).
- `src/domain/infrastructure/repository.rs` — La fase "Configuración inicial y creación de tareas" (f1) practica TaskRepository trait y su implementación en memoria (InMemoryTaskRepository) aqui: este archivo es el ejercicio (blueprint).

## Como saber que terminaste

```bash
cargo check
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Perfil
Chapter Backend, Especialidad Desarrollador, Tecnología Rust, Senior L2

### Brecha de conocimiento
Construir una API REST en Rust con Actix Web para gestionar tareas

### Misión / candidato
Exponer la gestion de tareas por HTTP

### Reto
- Tema: API REST de tareas con Rust y Actix
- Seniority: senior-l2
- Tipo: practical
- Título: Implementación de API REST para gestión de tareas
- Tiempo estimado: 12 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Configuración inicial y creación de tareas — objetivo: Configurar el entorno y crear el endpoint para añadir nuevas tareas. — entregable (NO resolver): Endpoint POST /tasks funcional que crea y almacena tareas en memoria.
- Fase 2: Listado y actualización de tareas — objetivo: Implementar endpoints para listar y actualizar tareas existentes. — entregable (NO resolver): Endpoints GET /tasks y PUT /tasks/{id} funcionales que listan y actualizan tareas, respectivamente.
- Fase 3: Optimización y consideraciones finales — objetivo: Optimizar la API y considerar decisiones de diseño. — entregable (NO resolver): Documentación que detalla las decisiones de diseño y posibles optimizaciones para la API.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (entrega de archivos).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Entrega de archivos
Entregá cada archivo con su ruta completa, respetando exactamente la estructura de rutas
indicada por los marcadores: un bloque por archivo, precedido por su ruta. No generes ZIPs
ni archivos adjuntos.
La entrega debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: Entregá los archivos completos al finalizar. No preguntes si el usuario
quiere recibirlos. Simplemente entregalos; No debes desplegar en el chat el resumen de lo que arreglaste, solo entregá los archivos.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del último archivo entregado
No preguntes si el usuario quiere los archivos: simplemente entregalos siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: Cargo.toml ===
[package]
name = "actix-web-skeleton"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.19.0"
anyhow = "1.0.104"
async-trait = "0.1.92"
dotenv = "0.15.0"
serde = { version = "1.0.229", features = ["derive"] }
sqlx = { version = "0.9.0", features = ["runtime-tokio", "macros"] }
thiserror = "1.0.69"
validator = "0.21.0"
tokio = { version = "1.54.1", features = ["macros", "rt-multi-thread"] }

[dev-dependencies]
actix-rt = "2.15.0"

// === ARCHIVO: src/contracts/types.rs ===
// Sample contract types
pub trait DomainContract {
    fn validate(&self) -> bool;
}

// === ARCHIVO: src/domain/mod.rs ===
pub mod task;
pub mod repository;
pub mod application;
pub mod infrastructure;
pub mod DESIGN;

// === ARCHIVO: src/contracts/mod.rs ===
// Contracts root module
pub mod types;

// === ARCHIVO: src/domain/task.rs ===
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

// === ARCHIVO: src/domain/repository.rs ===
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

// === ARCHIVO: src/domain/application/handlers.rs ===
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

// === ARCHIVO: src/domain/infrastructure/repository.rs ===
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

// === ARCHIVO: src/domain/DESIGN.md ===
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

// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpServer, Result as ActixResult};
use std::sync::{Arc, Mutex};

pub mod domain;
pub mod contracts;

use domain::task::Task;
use domain::task::TaskError;
use domain::repository::TaskRepository;
use domain::infrastructure::repository::InMemoryTaskRepository;

#[derive(Clone)]
struct AppState {
    repository: Arc<Mutex<dyn TaskRepository + Send + Sync>>,
}

async fn create_task(
    state: web::Data<AppState>,
    task: web::Json<CreateTaskRequest>,
) -> ActixResult<web::Json<Task>, TaskError> {
    let new_task = Task::new(task.title.clone(), task.description.clone())?;
    let mut repo = state.repository.lock().map_err(|_| TaskError::LockError)?;
    let saved = repo.save(new_task).await?;
    Ok(web::Json(saved))
}

async fn list_tasks(
    state: web::Data<AppState>,
) -> ActixResult<web::Json<Vec<Task>>, TaskError> {
    let repo = state.repository.lock().map_err(|_| TaskError::LockError)?;
    let tasks = repo.find_all().await?;
    Ok(web::Json(tasks))
}

async fn get_task(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> ActixResult<web::Json<Task>, TaskError> {
    let repo = state.repository.lock().map_err(|_| TaskError::LockError)?;
    let task = repo.find_by_id(path.into_inner()).await?;
    match task {
        Some(t) => Ok(web::Json(t)),
        None => Err(TaskError::NotFound),
    }
}

async fn update_task(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    update: web::Json<UpdateTaskRequest>,
) -> ActixResult<web::Json<Task>, TaskError> {
    let mut repo = state.repository.lock().map_err(|_| TaskError::LockError)?;
    let task_opt = repo.find_by_id(path.into_inner()).await?;
    
    let mut task = match task_opt {
        Some(t) => t,
        None => return Err(TaskError::NotFound),
    };
    
    if let Some(title) = &update.title {
        task = Task::new(title.clone(), task.description.clone())?;
    }
    if let Some(description) = &update.description {
        task = Task::new(task.title.clone(), description.clone())?;
    }
    if update.completed.unwrap_or(false) {
        task.complete();
    }
    
    let updated = repo.update(task).await?;
    Ok(web::Json(updated))
}

#[derive(serde::Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: String,
}

#[derive(serde::Deserialize)]
struct UpdateTaskRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let repository: Arc<Mutex<dyn TaskRepository + Send + Sync>> = 
        Arc::new(Mutex::new(InMemoryTaskRepository::new()));
    
    let app_state = AppState { repository };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(list_tasks))
            .route("/tasks/{id}", web::get().to(get_task))
            .route("/tasks/{id}", web::put().to(update_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// === ARCHIVO: src/domain/application/mod.rs ===
pub mod handlers;


// === ARCHIVO: src/domain/infrastructure/mod.rs ===
pub mod repository;

```
