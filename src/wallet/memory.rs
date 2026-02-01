use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};


use super::repository::{WalletRepository,WalletResult};

pub struct MemoryWalletRepository{
    store: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryWalletRepository {
    pub fn new() -> Self{
        Self { store: Arc::new(RwLock::new(HashMap::new())) }
    }
}

impl WalletRepository for MemoryWalletRepository {
    fn save_private_key(&self, address: String, private_key: Vec<u8>) -> WalletResult<()> {
        let mut db = self.store.write().unwrap();
        db.insert(address, private_key);
        Ok(())
    }
    
    fn load_private_key(&self, address: &str) -> WalletResult<Option<Vec<u8>>> {
        let db = self.store.read().unwrap();
        Ok(db.get(address).cloned())
    }
}