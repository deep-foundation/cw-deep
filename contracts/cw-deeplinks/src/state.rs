use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct DeeplinkState {
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    pub to: String,
}

pub const DEEPLINKS_KEY: &str = "deeplinks";
pub const DEEPLINKS: Map<u64, DeeplinkState> = Map::new(DEEPLINKS_KEY);

pub const NAMED_DEEPLINKS_KEY: &str = "named_deeplinks";
pub const NAMED_DEEPLINKS: Map<&str, DeeplinkState> = Map::new(NAMED_DEEPLINKS_KEY);

pub const ID_KEY: &str = "id";
pub const ID: Item<u64> = Item::new("ID_KEY");

pub const DELETED_IDS_KEY: &str = "deleted_ids";
pub const DELETED_IDS: Map<u64, bool> = Map::new(DELETED_IDS_KEY);

#[cw_serde]
pub struct Config {
    pub admins: Vec<Addr>,
    pub executors : Vec<Addr>
}

impl Config {
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admins.iter().any(|a| a.as_ref() == addr)
    }

    pub fn is_executor(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.executors.iter().any(|a| a.as_ref() == addr)
    }

    pub fn can_modify(&self, addr: &str) -> bool {
        self.is_admin(addr)
    }

    pub fn can_execute(&self, addr: &str) -> bool {
        self.is_executor(addr)
    }
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);


