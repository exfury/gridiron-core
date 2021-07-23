use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Info of each user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct UserInfo {
    pub amount: Uint128,
    pub reward_debt: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolInfo {
    pub alloc_point: u64,
    pub last_reward_block: u64,
    pub acc_per_share: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    // The xTRS TOKEN!
    pub xtrs_token: Addr,
    // xTRS balance
    pub xtrs_token_balance: Uint128,
    // Dev address.
    pub dev_addr: Addr,
    // Block number when bonus xTRS period ends.
    pub bonus_end_block: u64,
    // xTRS tokens created per block.
    pub tokens_per_block: Uint128,
    // Total allocation points. Must be the sum of all allocation points in all pools.
    pub total_alloc_point: u64,
    // The block number when xTRS mining starts.
    pub start_block: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POOL_INFO: Map<&Addr, PoolInfo> = Map::new("pool_info");

// first key part is token, second - depositor
pub const USER_INFO: Map<(&Addr, &Addr), UserInfo> = Map::new("user_info");
