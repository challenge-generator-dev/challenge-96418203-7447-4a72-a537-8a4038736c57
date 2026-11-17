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