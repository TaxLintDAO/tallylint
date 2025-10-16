use candid::error;
#[allow(unused_imports)]
use candid::{CandidType, Principal};

use serde::Serialize;
use std::collections::BTreeMap;

#[allow(unused_imports)]
use serde::Deserialize;

use super::domain::*;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct UserService {
  pub users: BTreeMap<Principal, UserProfile>,
  // ! important annotation for deserialize. if json not contains this, will
  // fill this with a empty one in rust code
  #[serde(default = "BTreeMap::new")]
  pub configs: BTreeMap<String, UserConfig>,
}

impl UserService {
  pub fn insert_user(
    &mut self,
    user: UserProfile,
  ) -> Result<UserProfile, String> {
    let owner = user.owner;
    match self.users.get(&owner) {
      Some(_) => Err(String::from("UserAlreadyExists")),
      None => {
        self.users.insert(owner, user.clone());
        Ok(user)
      }
    }
  }

  pub fn is_owner(&self, caller: &Principal) -> bool {
    matches!(self.users.get(caller), Some(u) if u.owner == *caller)
  }

  pub fn get_user(&self, principal: &Principal) -> Option<UserProfile> {
    self.users.get(principal).cloned()
  }

  #[allow(dead_code)]
  pub fn get_profile(&self, owner: &Principal) -> Option<&UserProfile> {
    self.users.get(owner)
  }

  pub fn user_quantity(&self) -> u32 {
    return self.users.len().try_into().unwrap_or_default();
  }

  pub fn add_default_config(&mut self, owner: &Principal) -> (){
    let config = UserConfig::new(
      //默认值为fifo
      "FIFO".to_string(),
      "USD".to_string(),
      "".to_string(),
      "".to_string(),
    );
    self.configs.insert(owner.to_string(), config);
  }
  /**
   * add or update user config
   */
  pub fn add_config(
    &mut self,
    owner: &Principal,
    data: UserConfig,
  ) -> Result<bool, String> {
    let operation = if self.configs.contains_key(&owner.to_string()) {
      "update"
    } else {
      "add"
    };

    self.configs.insert(owner.to_string(), data);

    // check the operation is successful
    match self.get_config(owner) {
      Ok(_) => Ok(true),
      Err(_) => Err(format!("{} failed", operation)),
    }
  }

  pub fn get_config(
    &mut self,
    owner: &Principal,
  ) -> Result<UserConfig, String> {
    // Try to get the config
    match self.configs.get(&owner.to_string()) {
      //如果有用户配置则返回
      Some(config) => Ok(config.clone()),
      // If the config does not exist, set a default config and return it
      // None => {
      //   let default_config = UserConfig::new("FIFO".to_string(), );
      //   self.update_config(&owner, default_config.clone());
      //   default_config
      // }
      None => {
        let config = UserConfig::new(
          //默认值为FIFO
          "FIFO".to_string(),
          "USD".to_string(),
          "".to_string(),
          "".to_string(),
        );
        self.configs.insert(owner.to_string(), config.clone());
        return Result::Ok(config.clone());
      }
    }
  }

  pub fn update_config(
    &mut self,
    owner: &Principal,
    data: UserConfig,
  ) -> String {
    self.configs.insert(owner.to_string(), data);
    return "update_config ok".to_string();
  }

  pub fn delete_config(&mut self, owner: &Principal) -> String {
    self.configs.remove(&owner.to_string());
    return "delete_config ok".to_string();
  }

  #[allow(dead_code)]
  pub fn new() -> Self {
    UserService {
      users: BTreeMap::new(),
      configs: BTreeMap::new(),
    }
  }
}
