use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  app_error::AppError,
  service::TaskService,
  task::{Task, TaskStatus},
};

#[derive(Deserialize)]
pub struct CreateTaskReq {
  name: String,
  data: serde_json::Value,
}

#[derive(Serialize)]
pub struct CreateTaskRes {
  id: Uuid,
  status: TaskStatus,
  created_at: i64,
  updated_at: i64,
}

pub async fn create_task(
  State(service): State<TaskService>,
  Json(task): axum::Json<CreateTaskReq>,
) -> Result<Json<CreateTaskRes>, AppError> {
  let task = Task::new(task.name, serde_json::to_vec(&task.data)?);
  service.create_task(&task)?;

  Ok(Json(CreateTaskRes {
    id: task.id,
    status: task.status,
    created_at: task.created_at.timestamp(),
    updated_at: task.updated_at.timestamp(),
  }))
}

pub async fn delete_task(
  State(service): State<TaskService>,
  Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<()>), AppError> {
  service.delete_task(id)?;
  Ok((StatusCode::OK, Json(())))
}
