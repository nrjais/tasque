use anyhow::Result;
use sled::IVec;
use uuid::Uuid;

const DB_NAME: &str = "main.tdb";

pub struct Storage {
  tree: sled::Db,
}

impl Storage {
  pub fn new() -> Result<Self> {
    let db = sled::open(DB_NAME)?;
    Ok(Self { tree: db })
  }

  pub fn get(&self, key: Uuid) -> Result<Option<IVec>> {
    let value = self.tree.get(key.as_bytes())?;
    Ok(value)
  }

  pub fn set(&self, key: Uuid, value: &[u8]) -> Result<()> {
    self.tree.insert(key.as_bytes(), value)?;
    Ok(())
  }

  pub fn delete(&self, key: Uuid) -> Result<()> {
    self.tree.remove(key.as_bytes())?;
    Ok(())
  }
}
