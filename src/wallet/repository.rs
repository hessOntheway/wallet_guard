use std::error::Error;

pub type WalletResult<T> = Result<T, Box<dyn Error>>;
pub trait WalletRepository: Send + Sync {
    fn save_private_key(&self, address: String,private_key: Vec<u8>)->WalletResult<()>;
    fn load_private_key(&self, address: &str) -> WalletResult<Option<Vec<u8>>>;
}