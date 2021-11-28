mod memory;

pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

/// The abstraction of storage does not care where it is stored,
/// but defines the interface how to interact with it
pub trait Storage {
    /// get key's value from HashTable
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// set key's value from HashTable, and return old value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// check if the key exists in the table
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// delete a key from the table
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// travers the hashtable and return all kv pairs
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// travers the hashtable and return an iterator of kv pair
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_show_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    fn test_basic_interface(store: impl Storage) {
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        let v1 = store.set("t1", "hello".into(), "world2".into());
        assert_eq!(v1, Ok(Some("world".into())));

        let v = store.get("t1", "hello".into());
        assert_eq!(v, Ok(Some("world2".into())));

        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert!(store.get("t2", "hello1").unwrap().is_none());

        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world2".into())));

        assert_eq!(Ok(None), store.del("t1", "hello1"));
        assert_eq!(Ok(None), store.del("t2", "hello"));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }

    #[allow(dead_code)]
    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq! {
          data,
          vec![
            Kvpair::new("k1", "v1".into()),
            Kvpair::new("k2", "v2".into())
          ]
        }
    }
}
