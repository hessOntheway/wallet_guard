use std::str::FromStr;

use crate::{service::wallet::generate_wallet, AppState};
use serde::{Serialize,Deserialize};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use alloy_primitives::{Address, U256, TxKind};
use alloy_signer_local::LocalSigner;
use alloy_rpc_types::TransactionRequest;

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

pub async fn create_wallet(State(state): State<AppState>) -> Result<Json<GenerateWalletResonse>, (StatusCode, String)> {
    let (mnemonic, _private_key, address) = generate_wallet();
    state.wallet_repo
        .save_private_key(address.clone(), _private_key.into())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save private key: {}", e)))?;
    Ok(Json(GenerateWalletResonse {
        mnemonic,   
        address,
    }))
}

pub async fn get_private_key(
    State(state): State<AppState>,
    Json(payload): Json<GetPrivateKeyRequest>,
) -> Result<Json<GetPrivateKeyResponse>, (StatusCode, String)> {

    let private_key_opt = state.wallet_repo
        .load_private_key(&payload.address)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load private key: {}", e)))?;
   
    let private_key = match private_key_opt {
        Some(v) => hex::encode(v),
        None => return Err((StatusCode::BAD_REQUEST, "address not exist".to_string()))
    };
    Ok(Json(GetPrivateKeyResponse { private_key }))
}

#[derive(Deserialize)]
pub struct SendTxRequest {
    pub from: String,
    pub to: String,
    pub value_wei: String,
}


pub async fn send_tx(
    State(state): State<AppState>,
    Json(req): Json<SendTxRequest>
) -> Result<Json<String>,(StatusCode,String)>
{

    let private_key_opt = state.wallet_repo
        .load_private_key(&req.from)
        .map_err(|e|(StatusCode::BAD_REQUEST,format!("Failed to load private key: {}",e)))?;

    let private_key = match private_key_opt{
        Some(v) => v,
        None => return Err((StatusCode::BAD_REQUEST, "address not exist".to_string()))
    };
    let _signer = LocalSigner::from_slice(&private_key).unwrap();
    let tx_request = TransactionRequest {
        from: Some(req.from.parse::<Address>().unwrap()),
        to: Some(TxKind::Call(req.to.parse::<Address>().unwrap())),
        value: Some(U256::from_str(&req.value_wei).unwrap()),
        ..Default::default()
    };
    let pending_tx = state.provider
        .send_transaction(tx_request)
        .await
        .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR,format!("Failed to send transaction: {}",e)))?;
    
    let tx_hash = pending_tx.tx_hash();
    let _js = Json(format!("0x{}", hex::encode(tx_hash)));
    Ok(Json(String::new()))
}
