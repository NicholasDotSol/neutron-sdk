use crate::bindings::dex::types::LimitOrderType;
use cosmwasm_std::Uint128;
use osmosis_std_derive::CosmwasmExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::types::{DepositOption, MultiHopRoute, PrecDec};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/neutron.dex.MsgDeposit")]
#[serde(rename_all = "snake_case")]
/// Deposit provides liquidity to a specific trading pair by depositing tokens
/// at a specific price into one or both sides of the pair in “a liquidity pool”
pub struct DepositMsg {
    /// The account to which PoolShares will be issued
    pub receiver: String,
    /// Denom for one side of the deposit
    pub token_a: String,
    /// Denom for the opposing side of the deposit
    pub token_b: String,
    /// Amounts of tokenA to deposit
    pub amounts_a: Vec<Uint128>,
    /// Amounts of tokenB to deposit
    pub amounts_b: Vec<Uint128>,
    /// Tick indexes to deposit at defined in terms of TokenA to TokenB (ie. TokenA is on the left)
    pub tick_indexes_a_to_b: Vec<i64>,
    /// Fees to use for each deposit
    pub fees: Vec<u64>,
    /// Additional deposit options
    pub options: Vec<DepositOption>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/neutron.dex.MsgWithdrawal")]
#[serde(rename_all = "snake_case")]
/// Withdraw is used to redeem PoolShares for the user’s pro-rata
/// portion of tokens within a liquidity pool. Users can withdraw from a pool at any time
pub struct WithdrawalMsg {
    /// The account to which the tokens are credited
    pub receiver: String,
    /// Denom for one side of the deposit
    pub token_a: String,
    /// Denom for the opposing side of the deposit
    pub token_b: String,
    /// Amount of shares to remove from each pool
    pub shares_to_remove: Vec<Uint128>,
    /// Tick indexes of the target LiquidityPools defined in terms of TokenA to TokenB
    /// (ie. TokenA is on the left)
    pub tick_indexes_a_to_b: Vec<i64>,
    /// Fee for the target LiquidityPools
    pub fees: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/neutron.dex.MsgPlaceLimitOrder")]
#[serde(rename_all = "snake_case")]
/// PlaceLimitOrder provides the primary mechanism for trading on the Duality Dex. Limit
/// orders can provide liquidity to the Dex (“Maker Limit Orders”) and/or can be used to
/// trade against preexisting liquidity (“Taker Limit Orders”)
pub struct PlaceLimitOrderMsg {
    /// Account to which TokenOut is credited or that will be allowed to
    /// withdraw or cancel a maker order
    pub receiver: String,
    /// Token being “sold”
    pub token_in: String,
    /// Token being “bought”
    pub token_out: String,
    /// Limit tick for a limit order, specified in terms of TokenIn to TokenOut
    pub tick_index_in_to_out: i64,
    /// Amount of TokenIn to be traded
    pub amount_in: Uint128,
    /// Type of limit order to be used. Must be one of:
    /// GOOD_TIL_CANCELLED, FILL_OR_KILL, IMMEDIATE_OR_CANCEL, JUST_IN_TIME, or GOOD_TIL_TIME
    pub order_type: LimitOrderType,
    // expirationTime is only valid if orderType == GOOD_TIL_TIME.
    /// Expiration time for order. Only valid for GOOD_TIL_TIME limit orders
    pub expiration_time: Option<u64>,
    /// Maximum amount of TokenB can be bought. For everything except JUST_IN_TIME OrderType
    pub max_amount_out: Option<Uint128>,
    /// Accepts standard decimals and decimals with scientific notation (ie. 1234.23E-7)
    pub limit_sell_price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/neutron.dex.MsgWithdrawFilledLimitOrder")]
#[serde(rename_all = "snake_case")]
/// WithdrawFilledLimitOrder. Once a limit order has been filled – either partially or in
/// its entirety, it can be withdrawn at any time. Withdrawing from a limit order credits
/// all available proceeds to the user. Withdraw can be called on a limit order multiple
/// times as new proceeds become available
pub struct WithdrawFilledLimitOrderMsg {
    /// TrancheKey for the target limit order
    pub tranche_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/neutron.dex.MsgCancelLimitOrder")]
#[serde(rename_all = "snake_case")]
/// CancelLimitOrder. Standard Taker limit orders (Good-til-cancelled & Good-til-Time)
/// can be canceled at any time if they have not been completely filled
pub struct CancelLimitOrderMsg {
    /// TrancheKey for the target limit order
    pub tranche_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/neutron.dex.MsgMultiHopSwap")]
#[serde(rename_all = "snake_case")]
/// MultiHopSwap provides a swapping mechanism to achieve better prices by routing
/// through a series of pools
pub struct MultiHopSwapMsg {
    /// Account to which TokenOut is credited
    pub receiver: String,
    /// Array of possible routes
    pub routes: Vec<MultiHopRoute>,
    /// Amount of TokenIn to swap
    pub amount_in: Uint128,
    /// Minimum price that that must be satisfied for a route to succeed
    pub exit_limit_price: PrecDec,
    /// If true all routes are run and the route with the best price is used
    pub pick_best_route: bool,
}

pub enum DexMsg {
    Deposit(DepositMsg),
    Withdrawal(WithdrawalMsg),
    PlaceLimitOrder(PlaceLimitOrderMsg),
    WithdrawFilledLimitOrder(WithdrawFilledLimitOrderMsg),
    CancelLimitOrder(CancelLimitOrderMsg),
    MultiHopSwap(MultiHopSwapMsg),
}