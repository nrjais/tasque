

use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sled::{
  transaction::{ConflictableTransactionResult, TransactionalTree},
  Transactional,
};
use uuid::Uuid;

const DB_NAME: &str = "main.tdb";
const ENTRIES: &str = "entries";
const IDX: &str = "idx";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Entry<T> {
  value: T,
  idx_key: Uuid,
}

pub struct IdxStorage<T> {
  entries: sled::Tree,
  idx: sled::Tree,
  _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> IdxStorage<T> {
  pub fn new() -> Result<Self> {
    let db = sled::open(DB_NAME)?;
    let entries = db.open_tree(ENTRIES)?;
    let idx = db.open_tree(IDX)?;
    Ok(Self {
      entries,
      idx,
      _phantom: std::marker::PhantomData,
    })
  }

  pub fn get(&self, key: Uuid) -> Result<Option<T>> {
    let value = self.entries.get(key.as_bytes())?;
    match value {
      Some(ref value) => {
        let entry: Entry<T> = bincode::deserialize(value)?;
        Ok(Some(entry.value))
      }
      None => Ok(None),
    }
  }

  pub fn set(&self, key: Uuid, value: &T) -> Result<()> {
    let entry = Entry {
      value,
      idx_key: Uuid::now_v7(),
    };

    let entry_bytes = bincode::serialize(&entry)?;
    let key_bytes = key.as_bytes();

    self.transaction(|(entries, idx)| {
      entries.insert(key_bytes, entry_bytes.as_slice())?;
      idx.insert(entry.idx_key.as_bytes(), key_bytes)?;
      Ok(())
    })?;

    Ok(())
  }

  pub fn delete(&self, key: Uuid) -> Result<()> {
    let key_bytes = key.as_bytes();
    let entry = self.entries.get(key_bytes)?;
    match entry {
      Some(entry) => {
        let entry: Entry<T> = bincode::deserialize(&entry)?;
        let idx_key_bytes = entry.idx_key.as_bytes();

        self.transaction(|(entries, idx)| {
          entries.remove(key_bytes)?;
          idx.remove(idx_key_bytes)?;
          Ok(())
        })?;

        Ok(())
      }
      None => Ok(()),
    }
  }

  #[inline]
  fn transaction<F>(&self, f: F) -> Result<()>
  where
    F: Fn(
      &(TransactionalTree, TransactionalTree),
    ) -> ConflictableTransactionResult<(), anyhow::Error>,
  {
    (&self.entries, &self.idx)
      .transaction(f)
      .map_err(|e| anyhow!("Transaction failed: {e}"))
  }
}
