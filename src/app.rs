use axum::{
  routing::{delete, post},
  Router,
};

use crate::{
  handlers::{create_task, delete_task},
  service::TaskService,
  IdxStorage,
};

pub fn create_app() -> Router {
  let db = IdxStorage::new().unwrap();
  let service = TaskService::new(db);

  Router::new()
    .route("/task", post(create_task))
    .route("/task/:id", delete(delete_task))
    .with_state(service)
}
