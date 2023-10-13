use candid::Principal;
use std::cell::RefCell;
// export apis below each
pub mod user;
pub mod wallet;
pub mod canister_api;

mod common;
use crate::common::context::CanisterContext;
thread_local! {
  static CONTEXT: RefCell<CanisterContext> = RefCell::default();
  static GOVERNANCE_ZHOU : Principal = Principal::from_text("ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae").unwrap();
  static GOVERNANCE_BTWL : Principal = Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae").unwrap();
}

// new auto did generate: in ic_cdk 0.10.0
// IMPORTANT :dont remove ref
// cargo hint to remove these is wrong. but must use below.
#[allow(unused_imports)]
use crate::user::domain::*;
#[allow(unused_imports)]
use crate::wallet::domain::*;
use ic_cdk::export_candid;
export_candid!();
