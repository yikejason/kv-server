use crate::{KvError, Kvpair, Value};

pub trait Storage {
    // 从一个 HashTable 里取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<String>, KvError>;

    // 从一个 HashTable 里设置一个 key 的 value, 返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;

    // 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;

    // 删除 HashTable 里的 key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    // 遍历 HashTable, 返回所有 kv pair
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;

    // 遍历 HashTable, 返回所有 kv pair Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
