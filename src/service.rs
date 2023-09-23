use std::sync::Arc;

use crate::{task::Task, IdxStorage};

#[derive(Clone)]
pub struct TaskService {
  db: Arc<IdxStorage<Task>>,
}

impl TaskService {
  pub fn new(db: IdxStorage<Task>) -> Self {
    Self { db: Arc::new(db) }
  }

  pub fn create_task(&self, task: &Task) -> Result<(), anyhow::Error> {
    self.db.set(task.id, task)?;
    Ok(())
  }

  pub fn delete_task(&self, id: uuid::Uuid) -> Result<(), anyhow::Error> {
    self.db.delete(id)?;
    Ok(())
  }
}
