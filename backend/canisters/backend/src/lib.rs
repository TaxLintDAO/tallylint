use crate::common::context::CanisterContext;
use ic_cdk::update;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use crate::http::http::HttpRequest;
use crate::common::guard::admin_guard;
thread_local! {
    pub static STATE: RefCell<CanisterContext> = RefCell::new(CanisterContext::new());
}

thread_local! {
    pub static BACKUP_DATA: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct BackupChunk {
    pub index: u32,       // 当前块的索引
    pub total: u32,       // 总块数
    pub data: Vec<u8>,    // 当前块的数据
}
#[update(guard = "admin_guard")]
fn backup_data(index: u32) -> Result<BackupChunk, String> {
    const CHUNK_SIZE: usize = 1024 * 1024; // 每个块的大小为 1 MB

    STATE.with(|state| {
        let state = state.borrow();
        let serialized = serde_json::to_vec(&*state).map_err(|e| e.to_string())?;

        let total_chunks = ((serialized.len() as f32) / (CHUNK_SIZE as f32)).ceil() as u32;
        let start = (index as usize) * CHUNK_SIZE;
        let end = std::cmp::min(start + CHUNK_SIZE, serialized.len());

        if start >= serialized.len() {
            return Err("Invalid chunk index".to_string());
        }

        let chunk_data = serialized[start..end].to_vec();

        Ok(BackupChunk {
            index,
            total: total_chunks,
            data: chunk_data,
        })
    })
}

#[update(guard = "admin_guard")]
fn restore_data(backup: Vec<u8>) -> Result<(), String> {
    let deserialized: CanisterContext = serde_json::from_slice(&backup).map_err(|e| e.to_string())?;
    STATE.with(|state| {
        *state.borrow_mut() = deserialized;
    });
    Ok(())
}

// #[update]
// fn backup_data() -> Result<Vec<u8>, String> {
//     STATE.with(|state| {
//         let state = state.borrow();
//         let serialized = serde_json::to_vec(&*state).map_err(|e| e.to_string())?;
//         Ok(serialized)
//     })
// }

// #[update]
// fn restore_data(backup: Vec<u8>) -> Result<(), String> {
//     let deserialized: CanisterContext = serde_json::from_slice(&backup).map_err(|e| e.to_string())?;
//     STATE.with(|state| {
//         *state.borrow_mut() = deserialized;
//     });
//     Ok(())
// }


pub mod common;
pub mod nns;
pub mod tools;
pub mod transaction;
pub mod user;
pub mod wallet;
pub mod http;

#[allow(unused_imports)]
use crate::common::context::TimeStamp;
#[allow(unused_imports)]
use crate::nns::api::*;
#[allow(unused_imports)]
use crate::nns::domain::*;
#[allow(unused_imports)]
use crate::transaction::domain::MySummary;
#[allow(unused_imports)]
use crate::transaction::domain::SimpleTransaction;
#[allow(unused_imports)]
use crate::transaction::domain::SyncTransactionCommand;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionB;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionF;
#[allow(unused_imports)]
use crate::user::domain::*;
#[allow(unused_imports)]
use crate::wallet::domain::*;
#[allow(unused_imports)]
use crate::wallet::service::RecordId;
#[allow(unused_imports)]
use crate::wallet::service::WalletAddress;
#[allow(unused_imports)]
use crate::wallet::service::WalletId;
#[allow(unused_imports)]
use ic_cdk::api::call::CallResult;
#[allow(unused_imports)]
use ic_cdk::api::management_canister::http_request::*;
#[allow(unused_imports)]
use ic_cdk::api::management_canister::{
  main::{
    canister_info, canister_status, create_canister, install_code,
    update_settings, CanisterInfoRequest, CanisterInfoResponse,
    CanisterStatusResponse,
  },
  provisional::{CanisterIdRecord, CanisterSettings},
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionId;

ic_cdk::export_candid!();
