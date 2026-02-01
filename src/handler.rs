use crate::service::wallet::{
    GenerateWalletResonse,
    generate_wallet
};

use axum::Json;

pub async fn create_wallet() -> Json<GenerateWalletResonse>{
    let (mnemonic, _private_key, address) = generate_wallet();
    Json(GenerateWalletResonse {
        mnemonic,   
        address,
    })
}