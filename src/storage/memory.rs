use dashmap::{mapref::one::Ref, DashMap};

use crate::{KvError, Kvpair, Storage, Value};

// 使用 DashMap 构建的 MemTable, 实现了 Storage trait
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    // 创建一个缺省的 MemTable
    pub fn new() -> Self {
        Self::default()
    }

    // 如果名为 name 的 hash table 不存在，则创建，否则返回
    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let ret: Option<Value> = table.get(key).map(|v| v.value().clone());
        Ok(ret)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let ret = table.insert(key, value);
        Ok(ret)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        let ret = table.contains_key(key);
        Ok(ret)
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let ret = table.remove(key).map(|(_k, v)| v);
        Ok(ret)
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        let ret = table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect();
        Ok(ret)
    }

    fn get_iter(&self, _table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}
