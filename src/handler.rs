use crate::{service::wallet::generate_wallet, wallet::WalletRepository};

use std::sync::Arc;
use serde::{Serialize,Deserialize};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
};

#[derive(Deserialize)]
pub struct GetPrivateKeyRequest {
    pub address: String,
}

#[derive(Serialize)]
pub struct GetPrivateKeyResponse{
    pub private_key: String
}

#[derive(Serialize)]
pub struct GenerateWalletResonse{
    pub mnemonic: String,
    pub address: String,
}

pub async fn create_wallet(State(wallet_repo): State<Arc<dyn WalletRepository>>) -> Result<Json<GenerateWalletResonse>, (StatusCode, String)> {
    let (mnemonic, _private_key, address) = generate_wallet();
    wallet_repo
        .save_private_key(address.clone(), _private_key.into())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save private key: {}", e)))?;
    Ok(Json(GenerateWalletResonse {
        mnemonic,   
        address,
    }))
}

pub async fn get_private_key(
    State(wallet_repo): State<Arc<dyn WalletRepository>>,
    Json(payload): Json<GetPrivateKeyRequest>,
) -> Result<Json<GetPrivateKeyResponse>, (StatusCode, String)> {

    let private_key_opt = wallet_repo
        .load_private_key(&payload.address)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load private key: {}", e)))?;
   
    let private_key = match private_key_opt {
        Some(v) => hex::encode(v),
        None => return Err((StatusCode::BAD_REQUEST, "address not exist".to_string()))
    };
    Ok(Json(GetPrivateKeyResponse { private_key }))
}