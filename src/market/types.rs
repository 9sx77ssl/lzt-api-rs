//! Auto-generated types for Lolzteam Public API: Market.
//! DO NOT EDIT -- run `cargo run --bin lzt-codegen -- generate`.

#![allow(clippy::all)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Query parameters for `Category.All`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryAllQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Payments.Fee`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsFeeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}


/// Query parameters for `Category.BattleNet`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeable_fn: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_btag: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "game[]")]
    pub game: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_bans: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_control: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_id: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Cart.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<MarketCategoryIdmodel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_queries: Option<bool>,
}


/// Query parameters for `Category.ChatGPT`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_balance_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_balance_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openai_tier[]")]
    pub openai_tier: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subscription[]")]
    pub subscription: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Profile.Claims`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


/// Query parameters for `Category.Discord`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2fa")]
    pub field_2fa: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "badge[]")]
    pub badge: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boosts_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boosts_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clans: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "condition[]")]
    pub condition: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifts: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language[]")]
    pub language: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin_clans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin_members: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_basic_credits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_full_credits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_orbs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_owner_clans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_servers: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin_clans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin_members: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_basic_credits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_full_credits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_orbs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_owner_clans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_servers: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nitro_type[]")]
    pub nitro_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_language[]")]
    pub not_language: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.EA`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_rank_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_rank_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "game[]")]
    pub game: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ban: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played_max: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_connected: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_connected: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_connected: Option<MarketYesNoNoMatterScheme>,
}


/// Query parameters for `Category.EpicGames`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "game[]")]
    pub game: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played_max: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_balance_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_balance_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rl_purchases: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.EscapeFromTarkov`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pve: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "version[]")]
    pub version: Option<Vec<String>>,
}


/// Query parameters for `List.Favorites`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListFavoritesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


/// Query parameters for `Category.Fortnite`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp_lmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp_lmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dance[]")]
    pub dance: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_vbmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_vbmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "glider[]")]
    pub glider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_vbmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_vbmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_trans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pickaxe[]")]
    pub pickaxe: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxe_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxe_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_vbmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_vbmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "platform[]")]
    pub platform: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_linkable: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_credits_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_credits_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rl_purchases: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skin[]")]
    pub skin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_vbmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_vbmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_linkable: Option<MarketYesNoNoMatterScheme>,
}


/// Query parameters for `Category.Gifts`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Hytale`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "edition[]")]
    pub edition: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Instagram`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_without_cookies: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Payments.Invoice.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}


/// Query parameters for `Payments.Invoice.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}


/// Query parameters for `Managing.GetLetters2`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetLetters2Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}


/// Query parameters for `Profile.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
}


/// Query parameters for `Category.Mihoyo`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinemas_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinemas_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellations_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellations_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ea: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eidolons_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eidolons_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_achievement_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_achievement_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_char_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_char_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "genshin_character[]")]
    pub genshin_character: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_character_constellations: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_character_constellations_max: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_currency_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_currency_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_weapon_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_weapon_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "genshin_weapon[]")]
    pub genshin_weapon: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_achievement_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_achievement_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_char_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_char_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "honkai_character[]")]
    pub honkai_character: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_character_eidolons: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_character_eidolons_max: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_currency_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_currency_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_weapon_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_weapon_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "honkai_weapon[]")]
    pub honkai_weapon: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_achievement_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_achievement_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_char_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_char_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "zenless_character[]")]
    pub zenless_character: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_character_cinemas: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_character_cinemas_max: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_currency_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_currency_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_weapon_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_weapon_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "zenless_weapon[]")]
    pub zenless_weapon: Option<Vec<i64>>,
}


/// Query parameters for `Category.Minecraft`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievement_hypixel_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievement_hypixel_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_details: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capes[]")]
    pub capes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capes_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capes_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_nickname: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dungeons: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypixel_ban: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypixel_ban_parsed: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypixel_skyblock_api_enabled: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_hypixel: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_hypixel_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legends: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_skyblock_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_skyblock_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecoins_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecoins_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_worth_hypixel_skyblock_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_worth_hypixel_skyblock_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname_length_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname_length_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rank_hypixel[]")]
    pub rank_hypixel: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Riot`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "agent[]")]
    pub agent: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buddy[]")]
    pub buddy: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "champion[]")]
    pub champion: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub champion_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub champion_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fa_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fa_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knife: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lol_not_region[]")]
    pub lol_not_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lol_rank[]")]
    pub lol_rank: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lol_region[]")]
    pub lol_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_smax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_smin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mythic_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mythic_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orange_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orange_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_rmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_rmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riot_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riot_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skin[]")]
    pub skin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_knife_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_knife_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valorant_not_region[]")]
    pub valorant_not_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valorant_rank_type[]")]
    pub valorant_rank_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valorant_region[]")]
    pub valorant_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_smax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_smin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weaponSkin[]")]
    pub weapon_skin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_rate_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_rate_min: Option<i64>,
}


/// Query parameters for `Category.Roblox`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "age_group[]")]
    pub age_group: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_verified: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_balance_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_balance_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_donations: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamepass_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamepass_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_robux_total_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_robux_total_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_price_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_price_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_age_group[]")]
    pub not_age_group: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsale_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsale_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_connected: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugc_limited_price_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugc_limited_price_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_connected: Option<MarketYesNoNoMatterScheme>,
}


/// Query parameters for `Category.SocialClub`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_cash_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_cash_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "game[]")]
    pub game: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Steam`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_games_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_games_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_map_rank: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_map_rmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_map_rmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_profile_rank_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_profile_rank_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_behavior_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_behavior_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_game_count_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_game_count_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_last_match_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_last_match_date_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_win_count_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_win_count_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elo_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elo_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_csgo_lvl_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_csgo_lvl_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_lvl_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_lvl_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "game[]")]
    pub game: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_purchase_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_purchase_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gift[]")]
    pub gift: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifts_purchase_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifts_purchase_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_activated_keys: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_faceit: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played_max: Option<BTreeMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingame_purchase_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingame_purchase_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_game: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_later: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_period_later: Option<MarketDatePeriodModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mafile: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "medal_id[]")]
    pub medal_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_operator_or: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm_ban: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_trans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_vac: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_hours_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_hours_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds_purchase_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds_purchase_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_gmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_gmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_deaths_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_deaths_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_kills_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_kills_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_vac_inv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solommr_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solommr_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_ban: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_limit: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vac[]")]
    pub vac: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vac_skip_game_check: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_count_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_count_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wingman_rmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wingman_rmin: Option<i64>,
}


/// Query parameters for `Managing.SteamValue`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    pub link: String,
}


/// Query parameters for `Category.Supercell`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_cup_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_cup_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_pass: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_wins_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_wins_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "brawler[]")]
    pub brawler: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawlers_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawlers_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_cup_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_cup_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_cup_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_cup_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_pass: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_wins_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_wins_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_year_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_year_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub king_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub king_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_brawlers_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_brawlers_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_cup_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_cup_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_pass: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_wins_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_wins_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_heroes_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_heroes_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_troops_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_troops_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heroes_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heroes_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spells_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spells_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_troops_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_troops_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town_hall_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town_hall_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Telegram`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_geo_spamblock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_after_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dc_id[]")]
    pub dc_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dig_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dig_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin_sub: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bot_active_users: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bots: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_channels: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chats: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversations: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gifts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gifts_convert_stars: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gifts_stars: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nft_gifts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stars: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin_sub: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bot_active_users: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bots: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_channels: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_chats: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_contacts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_conversations: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gifts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gifts_convert_stars: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gifts_stars: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_nft_gifts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stars: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_dc_id[]")]
    pub not_dc_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.TikTok`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coins_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coins_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_login: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
}


/// Query parameters for `Category.Uplay`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "game[]")]
    pub game: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_connected: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_ban: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_level_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_level_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "r6_operator[]")]
    pub r6_operator: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_operators_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_operators_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_rank_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_rank_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "r6_skin[]")]
    pub r6_skin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_smax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_smin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_connected: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<MarketYesNoNoMatterScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_connected: Option<MarketYesNoNoMatterScheme>,
}


/// Query parameters for `List.States`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<serde_json::Value>,
}


/// Query parameters for `List.User`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListUserQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<MarketCategoryIdmodel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delete_endDate")]
    pub delete_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delete_startDate")]
    pub delete_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_buyer_operation_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_delete_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_published_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paid_endDate")]
    pub paid_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paid_startDate")]
    pub paid_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "published_endDate")]
    pub published_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "published_startDate")]
    pub published_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


/// Query parameters for `List.Orders`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListOrdersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<MarketCategoryIdmodel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Payments.History`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id_lt: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_payment_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}


/// Query parameters for `List.Download`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListDownloadQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<MarketCategoryIdmodel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delete_endDate")]
    pub delete_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delete_startDate")]
    pub delete_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_buyer_operation_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_delete_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_published_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paid_endDate")]
    pub paid_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paid_startDate")]
    pub paid_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "published_endDate")]
    pub published_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "published_startDate")]
    pub published_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


/// Query parameters for `List.Viewed`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListViewedQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


/// Query parameters for `Category.Vpn`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service[]")]
    pub service: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Warface`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_rank_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_rank_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kredits_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kredits_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_kredits_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_kredits_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


/// Query parameters for `Category.Wot`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clan_role[]")]
    pub clan_role: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_clan_role[]")]
    pub not_clan_role: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_region[]")]
    pub not_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region[]")]
    pub region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tank[]")]
    pub tank: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmin: Option<i64>,
}


/// Query parameters for `Category.WotBlitz`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clan_role[]")]
    pub clan_role: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country[]")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_provider[]")]
    pub email_provider: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email_type[]")]
    pub email_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_clan_role[]")]
    pub not_clan_role: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_country[]")]
    pub not_country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_email_provider[]")]
    pub not_email_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_origin[]")]
    pub not_origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_public_tag_id[]")]
    pub not_public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_region[]")]
    pub not_region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_tag_id[]")]
    pub not_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "origin[]")]
    pub origin: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "public_tag_id[]")]
    pub public_tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region[]")]
    pub region: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag_id[]")]
    pub tag_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tank[]")]
    pub tank: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmin: Option<i64>,
}


/// Query parameters for `Managing.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
}


/// Query parameters for `Managing.Image`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingImageQuery {
    #[serde(rename = "type")]
    pub type_: String,
}


/// Query parameters for `Managing.SteamInventoryValue`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
}


/// Query parameters for `Managing.SteamPreview`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamPreviewQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsCreateRequest {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    pub day: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer: Option<String>,
    pub username_receiver: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsCreateResponse {
    pub auto_payment_id: i64,
    pub message: String,
    pub status: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsDeleteRequest {
    pub auto_payment_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponse {
    pub payments: AutoPaymentsListResponsePayments,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponsePayments {
    #[serde(rename = "1234567890")]
    pub field_1234567890: AutoPaymentsListResponsePayments1234567890,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponsePayments1234567890 {
    pub amount: String,
    pub auto_payment_id: i64,
    pub day: String,
    pub description: String,
    pub next_alert_date: i64,
    pub next_payment: i64,
    pub receiver: AutoPaymentsListResponsePayments1234567890Receiver,
    pub receiver_id: i64,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponsePayments1234567890Receiver {
    pub contest_count: i64,
    pub custom_title: String,
    pub fields: Vec<AutoPaymentsListResponsePayments1234567890ReceiverFieldsItem>,
    pub is_banned: i64,
    pub links: AutoPaymentsListResponsePayments1234567890ReceiverLinks,
    pub permissions: AutoPaymentsListResponsePayments1234567890ReceiverPermissions,
    pub trophy_count: i64,
    pub user_followers_count: i64,
    pub user_following_count: i64,
    pub user_group_id: i64,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_title: String,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponsePayments1234567890ReceiverFieldsItem {
    pub description: String,
    pub id: String,
    pub is_required: bool,
    pub position: String,
    pub title: String,
    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponsePayments1234567890ReceiverLinks {
    pub avatar: String,
    pub avatar_big: String,
    pub avatar_small: String,
    pub detail: String,
    pub followers: String,
    pub followings: String,
    pub ignore: String,
    pub permalink: String,
    pub timeline: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponsePayments1234567890ReceiverPermissions {
    pub edit: bool,
    pub follow: bool,
    pub ignore: bool,
    pub profile_post: bool,
}


pub type BatchRequest = Vec<BatchRequestItem>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    pub uri: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchResponse {
    pub jobs: BatchResponseJobs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<MarketRespSystemInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchResponseJobs {
    pub job_id: BatchResponseJobsJobId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchResponseJobsJobId {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_job_error")]
    pub job_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_job_result")]
    pub job_result: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartAddRequest {
    pub item_id: MarketItemIdmodel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartAddResponse {
    pub success: bool,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartDeleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<MarketItemIdmodel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartDeleteResponse {
    pub success: bool,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryBattleNetResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "battlenetBans")]
    pub battlenet_bans: String,
    #[serde(rename = "battlenetGames")]
    pub battlenet_games: CategoryBattleNetResponseItemsItemBattlenetGames,
    #[serde(rename = "battlenetTransactions")]
    pub battlenet_transactions: Vec<CategoryBattleNetResponseItemsItemBattlenetTransactionsItem>,
    pub battlenet_balance: String,
    #[serde(rename = "battlenet_bans")]
    pub battlenet_bans_2: String,
    pub battlenet_can_change_tag: i64,
    pub battlenet_change_full_name: i64,
    pub battlenet_converted_balance: i64,
    pub battlenet_country: String,
    pub battlenet_item_id: i64,
    pub battlenet_last_activity: i64,
    pub battlenet_mobile: i64,
    pub battlenet_parent_control: i64,
    pub battlenet_real_id_enabled: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryBattleNetResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    #[serde(rename = "displayConvertedBalance")]
    pub display_converted_balance: bool,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: CategoryBattleNetResponseItemsItemGuarantee,
    #[serde(rename = "hasOverwatch")]
    pub has_overwatch: bool,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryBattleNetResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItemBattlenetGames {
    #[serde(rename = "17459")]
    pub field_17459: CategoryBattleNetResponseItemsItemBattlenetGames17459,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItemBattlenetGames17459 {
    pub abbr: String,
    pub app_id: String,
    pub category_id: i64,
    pub img: String,
    pub internal_game_id: i64,
    pub ru: serde_json::Value,
    pub title: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItemBattlenetTransactionsItem {
    pub date: i64,
    #[serde(rename = "formattedTotal")]
    pub formatted_total: String,
    #[serde(rename = "productTitle")]
    pub product_title: String,
    pub total: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItemGuarantee {
    pub active: serde_json::Value,
    pub cancelled: serde_json::Value,
    pub class: String,
    pub duration: i64,
    #[serde(rename = "durationPhrase")]
    pub duration_phrase: String,
    #[serde(rename = "endDate")]
    pub end_date: serde_json::Value,
    #[serde(rename = "remainingTime")]
    pub remaining_time: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryChatGptResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponseItemsItem {
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryChatGptResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangeEmailPassword")]
    pub can_change_email_password: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewItemViews")]
    pub can_view_item_views: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub chatgpt_country: String,
    pub chatgpt_item_id: i64,
    pub chatgpt_phone: i64,
    pub chatgpt_register_date: i64,
    pub chatgpt_subscription: String,
    pub chatgpt_subscription_auto_renew: i64,
    pub chatgpt_subscription_ends: i64,
    #[serde(rename = "copyFormatData")]
    pub copy_format_data: CategoryChatGptResponseItemsItemCopyFormatData,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    #[serde(rename = "gptSubType")]
    pub gpt_sub_type: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isPersonalAccount")]
    pub is_personal_account: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: f64,
    #[serde(rename = "priceWithSellerFeeLabel")]
    pub price_with_seller_fee_label: String,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryChatGptResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    #[serde(rename = "uniqueKeyExists")]
    pub unique_key_exists: bool,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponseItemsItemCopyFormatData {
    pub title_link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryDiscordResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordResponseItemsItem {
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryDiscordResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    #[serde(rename = "discordAccountConditionLabel")]
    pub discord_account_condition_label: String,
    #[serde(rename = "discordLocaleTitle")]
    pub discord_locale_title: String,
    #[serde(rename = "discordNitroType")]
    pub discord_nitro_type: String,
    pub discord_admin_members_count: i64,
    pub discord_admin_servers: String,
    pub discord_admin_servers_count: i64,
    pub discord_available_boosts: i64,
    pub discord_billing: i64,
    pub discord_chat_count: i64,
    pub discord_condition: String,
    pub discord_gifts: i64,
    pub discord_item_id: i64,
    pub discord_locale: String,
    pub discord_nitro_end_date: i64,
    #[serde(rename = "discord_nitro_type")]
    pub discord_nitro_type_2: i64,
    pub discord_register_date: i64,
    pub discord_verified: i64,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryDiscordResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryEaResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryEaResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryEaResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub ea_al_level: i64,
    pub ea_al_rank_score: i64,
    pub ea_bans: Vec<serde_json::Value>,
    pub ea_country: String,
    pub ea_game_count: i64,
    pub ea_games: CategoryEaResponseItemsItemEaGames,
    pub ea_has_ban: i64,
    pub ea_id: i64,
    pub ea_item_id: i64,
    pub ea_last_activity: i64,
    pub ea_psn_connected: i64,
    pub ea_steam_connected: i64,
    pub ea_subscription: String,
    pub ea_subscription_end_date: i64,
    pub ea_username: String,
    pub ea_xbox_connected: i64,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryEaResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponseItemsItemEaGames {
    #[serde(rename = "apex-legends")]
    pub apex_legends: CategoryEaResponseItemsItemEaGamesApexLegends,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponseItemsItemEaGamesApexLegends {
    pub game_id: String,
    pub img: String,
    pub last_activity: i64,
    pub title: String,
    pub total_played: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryEpicGamesResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryEpicGamesResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "egBalance")]
    pub eg_balance: String,
    #[serde(rename = "egGameCount")]
    pub eg_game_count: i64,
    #[serde(rename = "egTransactions")]
    pub eg_transactions: Vec<CategoryEpicGamesResponseItemsItemEgTransactionsItem>,
    #[serde(rename = "eg_balance")]
    pub eg_balance_2: i64,
    pub eg_can_update_display_name: i64,
    pub eg_change_email: i64,
    pub eg_code_redemption_history: Vec<serde_json::Value>,
    pub eg_country: String,
    pub eg_coupons: Vec<serde_json::Value>,
    #[serde(rename = "eg_game_count")]
    pub eg_game_count_2: i64,
    pub eg_games: CategoryEpicGamesResponseItemsItemEgGames,
    pub eg_item_id: i64,
    pub eg_last_activity: i64,
    pub eg_next_change_email_date: i64,
    pub eg_payment_methods: Vec<serde_json::Value>,
    pub eg_rewards_balance: i64,
    pub eg_rewards_expiration_date: i64,
    pub eg_rl_purchases: i64,
    pub eg_username: String,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryEpicGamesResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponseItemsItemEgGames {
    #[serde(rename = "0")]
    pub field_0: CategoryEpicGamesResponseItemsItemEgGames0,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponseItemsItemEgGames0 {
    pub abbr: String,
    pub app_id: String,
    pub category_id: i64,
    pub hits_count: i64,
    pub img: String,
    pub internal_game_id: i64,
    pub link: String,
    pub ru: serde_json::Value,
    pub title: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponseItemsItemEgTransactionsItem {
    pub date: i64,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "presentmentTotal")]
    pub presentment_total: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryEscapeFromTarkovResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovResponseItemsItem {
    #[serde(rename = "accountDomain")]
    pub account_domain: String,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryEscapeFromTarkovResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryEscapeFromTarkovResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    #[serde(rename = "tarkovGameVersionPhrase")]
    pub tarkov_game_version_phrase: String,
    #[serde(rename = "tarkovKd")]
    pub tarkov_kd: i64,
    #[serde(rename = "tarkovRegionPhrase")]
    pub tarkov_region_phrase: String,
    #[serde(rename = "tarkovSecuredContainer")]
    pub tarkov_secured_container: String,
    pub tarkov_deaths: i64,
    pub tarkov_dollars: i64,
    pub tarkov_euros: i64,
    pub tarkov_exp: i64,
    pub tarkov_game_version: String,
    pub tarkov_item_id: i64,
    #[serde(rename = "tarkov_kd")]
    pub tarkov_kd_2: i64,
    pub tarkov_kills: i64,
    pub tarkov_last_activity: i64,
    pub tarkov_level: i64,
    pub tarkov_mail_forwarding: i64,
    pub tarkov_purchase_date: i64,
    pub tarkov_region: String,
    pub tarkov_register_date: i64,
    pub tarkov_rubles: i64,
    #[serde(rename = "tarkov_secured_container")]
    pub tarkov_secured_container_2: String,
    pub tarkov_sessions: i64,
    pub tarkov_side: String,
    pub tarkov_total_in_game: i64,
    pub tarkov_username: String,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryFortniteResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryFortniteResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub domain: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    #[serde(rename = "fortniteDance")]
    pub fortnite_dance: Vec<CategoryFortniteResponseItemsItemFortniteDanceItem>,
    #[serde(rename = "fortniteGliders")]
    pub fortnite_gliders: Vec<CategoryFortniteResponseItemsItemFortniteGlidersItem>,
    #[serde(rename = "fortnitePastSeasons")]
    pub fortnite_past_seasons: Vec<CategoryFortniteResponseItemsItemFortnitePastSeasonsItem>,
    #[serde(rename = "fortnitePickaxe")]
    pub fortnite_pickaxe: Vec<CategoryFortniteResponseItemsItemFortnitePickaxeItem>,
    #[serde(rename = "fortniteSkins")]
    pub fortnite_skins: Vec<CategoryFortniteResponseItemsItemFortniteSkinsItem>,
    #[serde(rename = "fortniteTransactions")]
    pub fortnite_transactions: Vec<CategoryFortniteResponseItemsItemFortniteTransactionsItem>,
    pub fortnite_balance: i64,
    pub fortnite_book_level: i64,
    pub fortnite_books_purchased: i64,
    pub fortnite_change_email: i64,
    pub fortnite_dance_count: i64,
    pub fortnite_glider_count: i64,
    pub fortnite_item_id: i64,
    pub fortnite_last_activity: i64,
    pub fortnite_last_trans_date: i64,
    pub fortnite_level: i64,
    pub fortnite_lifetime_wins: i64,
    pub fortnite_next_change_email_date: i64,
    pub fortnite_pickaxe_count: i64,
    pub fortnite_platform: String,
    pub fortnite_psn_linkable: i64,
    pub fortnite_register_date: i64,
    pub fortnite_rl_purchases: i64,
    pub fortnite_season_num: i64,
    pub fortnite_shop_dances_count: i64,
    pub fortnite_shop_gliders_count: i64,
    pub fortnite_shop_pickaxes_count: i64,
    pub fortnite_shop_skins_count: i64,
    pub fortnite_skin_count: i64,
    pub fortnite_xbox_linkable: i64,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryFortniteResponseItemsItemSeller,
    #[serde(rename = "shopCounts")]
    pub shop_counts: CategoryFortniteResponseItemsItemShopCounts,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemFortniteDanceItem {
    pub from_shop: i64,
    pub id: String,
    pub rarity: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemFortniteGlidersItem {
    pub from_shop: i64,
    pub id: String,
    pub rarity: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemFortnitePastSeasonsItem {
    #[serde(rename = "bookLevel")]
    pub book_level: i64,
    #[serde(rename = "numHighBracket")]
    pub num_high_bracket: i64,
    #[serde(rename = "numLowBracket")]
    pub num_low_bracket: i64,
    #[serde(rename = "numRoyalRoyales")]
    pub num_royal_royales: i64,
    #[serde(rename = "numWins")]
    pub num_wins: i64,
    #[serde(rename = "purchasedVIP")]
    pub purchased_vip: bool,
    #[serde(rename = "seasonLevel")]
    pub season_level: i64,
    #[serde(rename = "seasonNumber")]
    pub season_number: i64,
    #[serde(rename = "seasonXp")]
    pub season_xp: i64,
    #[serde(rename = "survivorPrestige")]
    pub survivor_prestige: i64,
    #[serde(rename = "survivorTier")]
    pub survivor_tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemFortnitePickaxeItem {
    pub from_shop: i64,
    pub id: String,
    pub rarity: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemFortniteSkinsItem {
    pub from_shop: i64,
    pub id: String,
    pub rarity: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemFortniteTransactionsItem {
    pub date: i64,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "presentmentTotal")]
    pub presentment_total: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponseItemsItemShopCounts {
    #[serde(rename = "shopDancesCount")]
    pub shop_dances_count: i64,
    #[serde(rename = "shopGlidersCount")]
    pub shop_gliders_count: i64,
    #[serde(rename = "shopPickaxesCount")]
    pub shop_pickaxes_count: i64,
    #[serde(rename = "shopSkinsCount")]
    pub shop_skins_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGamesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<CategoryGamesResponseGamesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<MarketRespSystemInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGamesResponseGamesItem {
    pub abbr: String,
    pub app_id: String,
    pub category_id: i64,
    pub img: String,
    pub ru: String,
    pub title: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryGiftsResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsResponseItemsItem {
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryGiftsResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    #[serde(rename = "giftsServiceName")]
    pub gifts_service_name: String,
    #[serde(rename = "giftsSubscriptionName")]
    pub gifts_subscription_name: String,
    pub gifts_duration: i64,
    pub gifts_item_id: i64,
    pub gifts_service: String,
    pub gifts_type: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryGiftsResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryHytaleResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponseItemsItem {
    pub allow_ask_discount: i64,
    pub auto_bump_period: i64,
    pub buyer: serde_json::Value,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangeEmailPassword")]
    pub can_change_email_password: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canManagePublicTag")]
    pub can_manage_public_tag: bool,
    #[serde(rename = "canNotBumpItemReason")]
    pub can_not_bump_item_reason: String,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItem")]
    pub can_resell_item: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewItemViews")]
    pub can_view_item_views: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    #[serde(rename = "canViewTempEmail")]
    pub can_view_temp_email: bool,
    pub category: CategoryHytaleResponseItemsItemCategory,
    pub category_id: i64,
    #[serde(rename = "copyFormatData")]
    pub copy_format_data: CategoryHytaleResponseItemsItemCopyFormatData,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub discount: bool,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    pub hytale_edition: String,
    pub hytale_item_id: i64,
    pub hytale_profiles: i64,
    #[serde(rename = "imagePreviewLinks")]
    pub image_preview_links: Vec<serde_json::Value>,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isPersonalAccount")]
    pub is_personal_account: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub max_discount_percent: i64,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub pending_deletion_date: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: f64,
    #[serde(rename = "priceWithSellerFeeLabel")]
    pub price_with_seller_fee_label: String,
    pub price_currency: String,
    pub public_tag: serde_json::Value,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryHytaleResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    #[serde(rename = "uniqueKeyExists")]
    pub unique_key_exists: bool,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponseItemsItemCategory {
    pub category_id: i64,
    pub category_name: String,
    pub category_title: String,
    pub category_url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponseItemsItemCopyFormatData {
    pub title_link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryInstagramResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryInstagramResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryInstagramResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    pub instagram_country: String,
    pub instagram_follow_count: i64,
    pub instagram_follower_count: i64,
    pub instagram_has_cookies: i64,
    pub instagram_id: String,
    pub instagram_item_id: i64,
    pub instagram_login_without_cookies: i64,
    pub instagram_mobile: i64,
    pub instagram_post_count: i64,
    pub instagram_register_date: i64,
    pub instagram_username: String,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryInstagramResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryListResponse {
    pub category: CategoryListResponseCategory,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryListResponseCategory {
    pub category_description: String,
    pub category_id: i64,
    pub category_title: String,
    pub links: CategoryListResponseCategoryLinks,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryListResponseCategoryLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryMihoyoResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryMihoyoResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryMihoyoResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    #[serde(rename = "genshinCharacters")]
    pub genshin_characters: Vec<CategoryMihoyoResponseItemsItemGenshinCharactersItem>,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "honkaiCharacters")]
    pub honkai_characters: Vec<CategoryMihoyoResponseItemsItemHonkaiCharactersItem>,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    #[serde(rename = "mihoyoLinkedAccounts")]
    pub mihoyo_linked_accounts: CategoryMihoyoResponseItemsItemMihoyoLinkedAccounts,
    #[serde(rename = "mihoyoLinkedAccountsString")]
    pub mihoyo_linked_accounts_string: String,
    #[serde(rename = "mihoyoRegionPhrase")]
    pub mihoyo_region_phrase: String,
    pub mihoyo_email: i64,
    pub mihoyo_genshin_abyss_process: String,
    pub mihoyo_genshin_achievement_count: i64,
    pub mihoyo_genshin_activity_days: i64,
    pub mihoyo_genshin_character_count: i64,
    pub mihoyo_genshin_constellations_count: i64,
    pub mihoyo_genshin_currency: i64,
    pub mihoyo_genshin_legendary_characters_count: i64,
    pub mihoyo_genshin_legendary_weapons_count: i64,
    pub mihoyo_genshin_level: i64,
    pub mihoyo_has_linked_accounts: i64,
    pub mihoyo_honkai_abyss_process: String,
    pub mihoyo_honkai_achievement_count: i64,
    pub mihoyo_honkai_activity_days: i64,
    pub mihoyo_honkai_character_count: i64,
    pub mihoyo_honkai_currency: i64,
    pub mihoyo_honkai_eidolons_count: i64,
    pub mihoyo_honkai_legendary_characters_count: i64,
    pub mihoyo_honkai_legendary_weapons_count: i64,
    pub mihoyo_honkai_level: i64,
    pub mihoyo_id: i64,
    pub mihoyo_item_id: i64,
    pub mihoyo_last_activity: i64,
    pub mihoyo_region: String,
    pub mihoyo_zenless_abyss_process: String,
    pub mihoyo_zenless_achievement_count: i64,
    pub mihoyo_zenless_activity_days: i64,
    pub mihoyo_zenless_character_count: i64,
    pub mihoyo_zenless_cinemas_count: i64,
    pub mihoyo_zenless_currency: i64,
    pub mihoyo_zenless_legendary_characters_count: i64,
    pub mihoyo_zenless_legendary_weapons_count: i64,
    pub mihoyo_zenless_level: i64,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryMihoyoResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
    #[serde(rename = "zenlessCharacters")]
    pub zenless_characters: Vec<CategoryMihoyoResponseItemsItemZenlessCharactersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemGenshinCharactersItem {
    pub actived_constellation_num: i64,
    pub background: String,
    pub costumes: Vec<serde_json::Value>,
    pub element: String,
    pub external: serde_json::Value,
    pub fetter: i64,
    pub icon: String,
    pub id: i64,
    pub image: String,
    pub level: i64,
    pub name: String,
    pub rarity: i64,
    pub reliquaries: Vec<CategoryMihoyoResponseItemsItemGenshinCharactersItemReliquariesItem>,
    pub weapon: CategoryMihoyoResponseItemsItemGenshinCharactersItemWeapon,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemGenshinCharactersItemReliquariesItem {
    pub icon: String,
    pub id: i64,
    pub level: i64,
    pub name: String,
    pub pos: i64,
    pub pos_name: String,
    pub rarity: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemGenshinCharactersItemWeapon {
    pub affix_level: i64,
    pub desc: String,
    pub icon: String,
    pub id: i64,
    pub level: i64,
    pub name: String,
    pub promote_level: i64,
    pub rarity: i64,
    #[serde(rename = "type")]
    pub type_: i64,
    pub type_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItem {
    pub base_type: i64,
    pub element: String,
    #[serde(rename = "elementImage")]
    pub element_image: String,
    pub equip: CategoryMihoyoResponseItemsItemHonkaiCharactersItemEquip,
    pub figure_path: String,
    pub icon: String,
    pub id: i64,
    pub image: String,
    pub level: i64,
    pub name: String,
    pub ornaments: Vec<CategoryMihoyoResponseItemsItemHonkaiCharactersItemOrnamentsItem>,
    pub rank: i64,
    pub rarity: i64,
    pub relics: Vec<CategoryMihoyoResponseItemsItemHonkaiCharactersItemRelicsItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemEquip {
    pub desc: String,
    pub icon: String,
    pub id: i64,
    pub level: i64,
    pub name: String,
    pub rank: i64,
    pub rarity: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemOrnamentsItem {
    pub desc: String,
    pub icon: String,
    pub id: i64,
    pub level: i64,
    pub main_property: CategoryMihoyoResponseItemsItemHonkaiCharactersItemOrnamentsItemMainProperty,
    pub name: String,
    pub pos: i64,
    pub properties: Vec<CategoryMihoyoResponseItemsItemHonkaiCharactersItemOrnamentsItemPropertiesItem>,
    pub rarity: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemOrnamentsItemMainProperty {
    pub property_type: i64,
    pub times: i64,
    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemOrnamentsItemPropertiesItem {
    pub property_type: i64,
    pub times: i64,
    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemRelicsItem {
    pub desc: String,
    pub icon: String,
    pub id: i64,
    pub level: i64,
    pub main_property: CategoryMihoyoResponseItemsItemHonkaiCharactersItemRelicsItemMainProperty,
    pub name: String,
    pub pos: i64,
    pub properties: Vec<CategoryMihoyoResponseItemsItemHonkaiCharactersItemRelicsItemPropertiesItem>,
    pub rarity: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemRelicsItemMainProperty {
    pub property_type: i64,
    pub times: i64,
    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemHonkaiCharactersItemRelicsItemPropertiesItem {
    pub property_type: i64,
    pub times: i64,
    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemMihoyoLinkedAccounts {
    pub legacy: bool,
    pub links: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemZenlessCharactersItem {
    pub avatar_profession: i64,
    pub camp_name_mi18n: String,
    #[serde(rename = "elementIcon")]
    pub element_icon: String,
    pub element_type: i64,
    pub full_name_mi18n: String,
    pub id: i64,
    pub level: i64,
    pub name: String,
    pub name_mi18n: String,
    #[serde(rename = "professionIcon")]
    pub profession_icon: String,
    pub rank: i64,
    pub rarity: i64,
    #[serde(rename = "rarityIcon")]
    pub rarity_icon: String,
    pub weapon: CategoryMihoyoResponseItemsItemZenlessCharactersItemWeapon,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemZenlessCharactersItemWeapon {
    pub icon: String,
    pub id: i64,
    pub level: i64,
    pub main_properties: Vec<CategoryMihoyoResponseItemsItemZenlessCharactersItemWeaponMainPropertiesItem>,
    pub name: String,
    pub profession: i64,
    pub properties: Vec<CategoryMihoyoResponseItemsItemZenlessCharactersItemWeaponPropertiesItem>,
    pub rarity: i64,
    #[serde(rename = "rarityIcon")]
    pub rarity_icon: String,
    pub star: i64,
    #[serde(rename = "starIcon")]
    pub star_icon: String,
    pub talent_content: String,
    pub talent_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemZenlessCharactersItemWeaponMainPropertiesItem {
    pub base: String,
    pub property_id: i64,
    pub property_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponseItemsItemZenlessCharactersItemWeaponPropertiesItem {
    pub base: String,
    pub property_id: i64,
    pub property_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryMinecraftResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryMinecraftResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryMinecraftResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    #[serde(rename = "minecraftHasPaidLicense")]
    pub minecraft_has_paid_license: bool,
    pub minecraft_bedrock: i64,
    pub minecraft_can_change_nickname: i64,
    pub minecraft_capes: Vec<serde_json::Value>,
    pub minecraft_capes_count: i64,
    pub minecraft_country: String,
    pub minecraft_created_at: i64,
    pub minecraft_dungeons: i64,
    pub minecraft_email_reset_date: i64,
    pub minecraft_hypixel_achievement: i64,
    pub minecraft_hypixel_ban: i64,
    pub minecraft_hypixel_ban_reason: String,
    pub minecraft_hypixel_last_login: i64,
    pub minecraft_hypixel_level: i64,
    pub minecraft_hypixel_rank: String,
    pub minecraft_hypixel_skyblock_level: i64,
    pub minecraft_hypixel_skyblock_net_worth: i64,
    pub minecraft_id: String,
    pub minecraft_item_id: i64,
    pub minecraft_java: i64,
    pub minecraft_legends: i64,
    pub minecraft_nickname: String,
    pub minecraft_skin: String,
    pub minecraft_subscription_auto_renew: i64,
    pub minecraft_subscription_ends: i64,
    pub minecraft_subscription_name: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryMinecraftResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_params: Option<CategoryParamsResponseBaseParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CategoryParamsResponseCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<CategoryParamsResponseParamsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<MarketRespSystemInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponseBaseParams {
    #[serde(rename = "0")]
    pub field_0: CategoryParamsResponseBaseParams0,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponseBaseParams0 {
    pub description: String,
    pub input: String,
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponseCategory {
    pub account_price_min: i64,
    pub add_item_available: i64,
    pub available_temp_email: i64,
    pub buy_without_validation: i64,
    pub can_be_resold: i64,
    pub category_description_html: String,
    pub category_description_html_en: String,
    pub category_h1_html_en: String,
    pub category_id: i64,
    pub category_login_url: String,
    pub category_name: String,
    pub category_order: i64,
    pub category_title: String,
    pub category_url: String,
    pub check_button_enabled: i64,
    pub checker_enabled: i64,
    pub cookies: String,
    pub display_in_list: i64,
    pub guest_hidden: i64,
    pub has_account_link: i64,
    pub has_guarantee: i64,
    pub login_type: String,
    pub mass_upload_item_available: i64,
    pub max_invalid_upload_tries: i64,
    pub recovery_link: String,
    pub require_eld_for_native_accs: i64,
    pub require_email_login_data: i64,
    pub require_temp_email: i64,
    pub require_video_recording: i64,
    pub resale_duration_limit_days: i64,
    pub sub_category_id: i64,
    pub support_email_login_data: i64,
    pub support_personal_proxy: i64,
    pub support_temp_email: i64,
    pub top_queries: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponseParamsItem {
    pub description: String,
    pub input: String,
    pub name: String,
    pub values: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryRiotResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryRiotResponseItemsItemAccountLinksItem>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryRiotResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    #[serde(rename = "lolInventory")]
    pub lol_inventory: CategoryRiotResponseItemsItemLolInventory,
    #[serde(rename = "lolRegionPhrase")]
    pub lol_region_phrase: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub riot_account_verified: i64,
    pub riot_country: String,
    pub riot_email_verified: i64,
    pub riot_id: String,
    pub riot_item_id: i64,
    pub riot_last_activity: i64,
    pub riot_lol_champion_count: i64,
    pub riot_lol_level: i64,
    pub riot_lol_rank: String,
    pub riot_lol_rank_win_rate: i64,
    pub riot_lol_region: String,
    pub riot_lol_skin_count: i64,
    pub riot_lol_wallet_blue: i64,
    pub riot_lol_wallet_mythic: i64,
    pub riot_lol_wallet_orange: i64,
    pub riot_lol_wallet_riot: i64,
    pub riot_password_change: i64,
    pub riot_phone_verified: i64,
    pub riot_username: String,
    pub riot_valorant_agent_count: i64,
    pub riot_valorant_inventory_value: i64,
    pub riot_valorant_knife: i64,
    pub riot_valorant_last_rank: i64,
    pub riot_valorant_level: i64,
    pub riot_valorant_previous_rank: i64,
    pub riot_valorant_rank: i64,
    pub riot_valorant_rank_type: String,
    pub riot_valorant_region: String,
    pub riot_valorant_skin_count: i64,
    pub riot_valorant_wallet_fa: i64,
    pub riot_valorant_wallet_rp: i64,
    pub riot_valorant_wallet_vp: i64,
    pub rub_price: i64,
    pub seller: CategoryRiotResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    #[serde(rename = "valorantInventory")]
    pub valorant_inventory: CategoryRiotResponseItemsItemValorantInventory,
    #[serde(rename = "valorantLastRankTitle")]
    pub valorant_last_rank_title: String,
    #[serde(rename = "valorantPreviousRankTitle")]
    pub valorant_previous_rank_title: String,
    #[serde(rename = "valorantRankImgPath")]
    pub valorant_rank_img_path: String,
    #[serde(rename = "valorantRankTitle")]
    pub valorant_rank_title: String,
    #[serde(rename = "valorantRegionPhrase")]
    pub valorant_region_phrase: String,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponseItemsItemLolInventory {
    #[serde(rename = "Champion")]
    pub champion: Vec<i64>,
    #[serde(rename = "Skin")]
    pub skin: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponseItemsItemValorantInventory {
    #[serde(rename = "Agent")]
    pub agent: Vec<String>,
    #[serde(rename = "Buddy")]
    pub buddy: Vec<String>,
    #[serde(rename = "WeaponSkins")]
    pub weapon_skins: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryRobloxResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryRobloxResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryRobloxResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    #[serde(rename = "creditBalance")]
    pub credit_balance: String,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    #[serde(rename = "robloxGameDonations")]
    pub roblox_game_donations: Vec<CategoryRobloxResponseItemsItemRobloxGameDonationsItem>,
    #[serde(rename = "robloxGameDonationsDetails")]
    pub roblox_game_donations_details: Vec<CategoryRobloxResponseItemsItemRobloxGameDonationsDetailsItem>,
    #[serde(rename = "robloxLinkedAccounts")]
    pub roblox_linked_accounts: String,
    pub roblox_age_verified: i64,
    pub roblox_country: String,
    pub roblox_credit_balance: i64,
    pub roblox_email_verified: i64,
    pub roblox_followers: i64,
    pub roblox_friends: i64,
    #[serde(rename = "roblox_game_donations")]
    pub roblox_game_donations_2: String,
    pub roblox_game_pass_total_robux: i64,
    pub roblox_id: i64,
    pub roblox_incoming_robux_total: i64,
    pub roblox_inventory_price: i64,
    pub roblox_item_id: i64,
    pub roblox_limited_price: i64,
    pub roblox_psn_connected: i64,
    pub roblox_register_date: i64,
    pub roblox_robux: i64,
    pub roblox_subscription: String,
    pub roblox_subscription_auto_renew: i64,
    pub roblox_subscription_end_date: i64,
    pub roblox_ugc_limited_price: i64,
    pub roblox_username: String,
    pub roblox_verified: i64,
    pub roblox_xbox_connected: i64,
    pub rub_price: i64,
    pub seller: CategoryRobloxResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponseItemsItemRobloxGameDonationsDetailsItem {
    pub amount: i64,
    pub product: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponseItemsItemRobloxGameDonationsItem {
    pub amount: i64,
    pub id: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategorySocialClubResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategorySocialClubResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategorySocialClubResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub socialclub_bank_cash: i64,
    pub socialclub_cash: i64,
    pub socialclub_games: Vec<CategorySocialClubResponseItemsItemSocialclubGamesItem>,
    pub socialclub_has_gtav: i64,
    pub socialclub_has_rdr2: i64,
    pub socialclub_item_id: i64,
    pub socialclub_last_activity: i64,
    pub socialclub_level: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponseItemsItemSocialclubGamesItem {
    pub abbr: String,
    pub app_id: String,
    pub category_id: i64,
    #[serde(rename = "defaultPlatform")]
    pub default_platform: String,
    pub id: i64,
    pub img: String,
    pub internal_game_id: i64,
    #[serde(rename = "lastSeen")]
    pub last_seen: String,
    pub name: String,
    pub platform: String,
    pub ru: serde_json::Value,
    pub title: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategorySteamResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategorySteamResponseItemsItemAccountLinksItem>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategorySteamResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    #[serde(rename = "chineseAccount")]
    pub chinese_account: bool,
    #[serde(rename = "cs2MapsRanks")]
    pub cs2maps_ranks: Vec<serde_json::Value>,
    #[serde(rename = "cs2PremierElo")]
    pub cs2premier_elo: Vec<serde_json::Value>,
    #[serde(rename = "cs2RankExpired")]
    pub cs2rank_expired: bool,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    #[serde(rename = "displayConvertedBalance")]
    pub display_converted_balance: bool,
    #[serde(rename = "dota2CalibrationWarning")]
    pub dota2calibration_warning: bool,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: CategorySteamResponseItemsItemGuarantee,
    #[serde(rename = "hasCs2")]
    pub has_cs2: bool,
    #[serde(rename = "hasDota2")]
    pub has_dota2: bool,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "hasPossibleBanInDota2")]
    pub has_possible_ban_in_dota2: bool,
    #[serde(rename = "hasPubg")]
    pub has_pubg: bool,
    #[serde(rename = "hasRust")]
    pub has_rust: bool,
    #[serde(rename = "hasTf2")]
    pub has_tf2: bool,
    #[serde(rename = "inventoryValue")]
    pub inventory_value: Vec<serde_json::Value>,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategorySteamResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    #[serde(rename = "steamCs2Medals")]
    pub steam_cs2medals: Vec<serde_json::Value>,
    #[serde(rename = "steamData")]
    pub steam_data: CategorySteamResponseItemsItemSteamData,
    #[serde(rename = "steamDota2WinRate")]
    pub steam_dota2win_rate: i64,
    #[serde(rename = "steamLifetimeTradeBan")]
    pub steam_lifetime_trade_ban: bool,
    #[serde(rename = "steamRelevantGameCount")]
    pub steam_relevant_game_count: i64,
    #[serde(rename = "steamTransactions")]
    pub steam_transactions: Vec<CategorySteamResponseItemsItemSteamTransactionsItem>,
    pub steam_balance: String,
    pub steam_bans: String,
    pub steam_cards_count: i64,
    pub steam_cards_games: i64,
    pub steam_community_ban: i64,
    pub steam_converted_balance: i64,
    pub steam_country: String,
    pub steam_cs2_ban_date: i64,
    pub steam_cs2_ban_date_active: bool,
    pub steam_cs2_ban_type: i64,
    pub steam_cs2_inv_value: i64,
    pub steam_cs2_last_activity: i64,
    pub steam_cs2_last_launched: i64,
    pub steam_cs2_premier_elo: i64,
    pub steam_cs2_profile_rank: i64,
    pub steam_cs2_rank_id: i64,
    pub steam_cs2_win_count: i64,
    pub steam_cs2_wingman_rank_id: i64,
    pub steam_dota2_behavior: i64,
    pub steam_dota2_game_count: i64,
    pub steam_dota2_inv_value: i64,
    pub steam_dota2_last_match_date: i64,
    pub steam_dota2_lose_count: i64,
    pub steam_dota2_solo_mmr: i64,
    pub steam_dota2_win_count: i64,
    pub steam_dst_inv_value: i64,
    pub steam_faceit_level: i64,
    pub steam_friend_count: i64,
    pub steam_full_games: CategorySteamResponseItemsItemSteamFullGames,
    pub steam_game_count: i64,
    pub steam_gift_count: i64,
    pub steam_has_activated_keys: i64,
    pub steam_hours_played_recently: String,
    pub steam_inv_value: i64,
    pub steam_is_limited: i64,
    pub steam_item_id: i64,
    pub steam_kf2_inv_value: i64,
    pub steam_last_activity: i64,
    pub steam_last_transaction_date: i64,
    pub steam_level: i64,
    pub steam_limit_spent: String,
    pub steam_market: i64,
    pub steam_market_ban_end_date: i64,
    pub steam_market_restrictions: i64,
    pub steam_mfa: i64,
    pub steam_points: i64,
    pub steam_pubg_inv_value: i64,
    pub steam_register_date: i64,
    #[serde(rename = "steam_relevant_game_count")]
    pub steam_relevant_game_count_2: i64,
    pub steam_rust_deaths: i64,
    pub steam_rust_inv_value: i64,
    pub steam_rust_kill_player: i64,
    pub steam_steam_inv_value: i64,
    pub steam_tf2_inv_value: i64,
    pub steam_total_games_rub: i64,
    pub steam_total_gifts_rub: i64,
    pub steam_total_ingame_rub: i64,
    pub steam_total_purchased_rub: i64,
    pub steam_total_refunds_rub: i64,
    pub steam_unturned_inv_value: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemGuarantee {
    pub active: serde_json::Value,
    pub cancelled: serde_json::Value,
    pub class: String,
    pub duration: i64,
    #[serde(rename = "durationPhrase")]
    pub duration_phrase: String,
    #[serde(rename = "endDate")]
    pub end_date: serde_json::Value,
    #[serde(rename = "remainingTime")]
    pub remaining_time: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemSteamData {
    pub steam_ban_type_id: Vec<serde_json::Value>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemSteamFullGames {
    pub list: CategorySteamResponseItemsItemSteamFullGamesList,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemSteamFullGamesList {
    #[serde(rename = "730")]
    pub field_730: CategorySteamResponseItemsItemSteamFullGamesList730,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemSteamFullGamesList730 {
    pub abbr: String,
    pub appid: i64,
    pub img: String,
    pub internal_game_id: i64,
    #[serde(rename = "parentGameId")]
    pub parent_game_id: i64,
    pub playtime_forever: f64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponseItemsItemSteamTransactionsItem {
    pub amount: String,
    pub date: String,
    pub product: String,
    pub source: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategorySupercellResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategorySupercellResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategorySupercellResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategorySupercellResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    #[serde(rename = "supercellBrawlers")]
    pub supercell_brawlers: Vec<serde_json::Value>,
    pub supercell_arena: String,
    pub supercell_brawler_count: i64,
    pub supercell_builder_hall_cup_count: i64,
    pub supercell_builder_hall_level: i64,
    pub supercell_id: String,
    pub supercell_item_id: i64,
    pub supercell_king_level: i64,
    pub supercell_laser_battle_pass: i64,
    pub supercell_laser_level: i64,
    pub supercell_laser_trophies: i64,
    pub supercell_laser_victories: i64,
    pub supercell_last_activity: i64,
    pub supercell_legendary_brawler_count: i64,
    pub supercell_magic_battle_pass: i64,
    pub supercell_magic_level: i64,
    pub supercell_magic_trophies: i64,
    pub supercell_magic_victories: i64,
    pub supercell_phone: i64,
    pub supercell_scroll_battle_pass: i64,
    pub supercell_scroll_level: i64,
    pub supercell_scroll_trophies: i64,
    pub supercell_scroll_victories: i64,
    pub supercell_systems: String,
    pub supercell_total_builder_heroes_level: i64,
    pub supercell_total_builder_troops_level: i64,
    pub supercell_total_heroes_level: i64,
    pub supercell_total_spells_level: i64,
    pub supercell_total_troops_level: i64,
    pub supercell_town_hall_level: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryTelegramResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryTelegramResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: CategoryTelegramResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: Vec<serde_json::Value>,
    pub telegram_admin_count: i64,
    pub telegram_admin_subs_count: i64,
    pub telegram_birthday: i64,
    pub telegram_channels_count: i64,
    pub telegram_chats_count: i64,
    pub telegram_contacts_count: i64,
    pub telegram_conversations_count: i64,
    pub telegram_country: String,
    pub telegram_group_counters: CategoryTelegramResponseItemsItemTelegramGroupCounters,
    pub telegram_id_count: i64,
    pub telegram_item_id: i64,
    pub telegram_last_seen: i64,
    pub telegram_password: i64,
    pub telegram_premium: i64,
    pub telegram_premium_expires: i64,
    pub telegram_spam_block: serde_json::Value,
    pub telegram_stars_count: i64,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponseItemsItemTelegramGroupCounters {
    pub admin: i64,
    pub channels: i64,
    pub chats: i64,
    pub conversations: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryTikTokResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponseItemsItem {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<CategoryTikTokResponseItemsItemAccountLinksItem>,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryTikTokResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryTikTokResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub tt_coins: i64,
    pub tt_cookie_login: i64,
    pub tt_countries: String,
    #[serde(rename = "tt_createTime")]
    pub tt_create_time: i64,
    pub tt_followers: i64,
    pub tt_following: i64,
    #[serde(rename = "tt_hasEmail")]
    pub tt_has_email: i64,
    #[serde(rename = "tt_hasLivePermission")]
    pub tt_has_live_permission: i64,
    #[serde(rename = "tt_hasMobile")]
    pub tt_has_mobile: i64,
    pub tt_id: i64,
    pub tt_item_id: i64,
    pub tt_likes: i64,
    pub tt_permalink: String,
    #[serde(rename = "tt_privateAccount")]
    pub tt_private_account: i64,
    pub tt_screen_name: String,
    pub tt_top_country: String,
    #[serde(rename = "tt_uniqueId")]
    pub tt_unique_id: String,
    pub tt_verified: i64,
    pub tt_videos: i64,
    pub update_stat_date: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponseItemsItemAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryUplayResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponseItemsItem {
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryUplayResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    #[serde(rename = "emailLoginUrl")]
    pub email_login_url: String,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    #[serde(rename = "r6Operators")]
    pub r6operators: Vec<CategoryUplayResponseItemsItemR6operatorsItem>,
    #[serde(rename = "r6Skins")]
    pub r6skins: Vec<serde_json::Value>,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryUplayResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    #[serde(rename = "uplayLinkedAccounts")]
    pub uplay_linked_accounts: String,
    #[serde(rename = "uplayR6Rank")]
    pub uplay_r6rank: String,
    pub uplay_country: String,
    pub uplay_created_date: i64,
    pub uplay_game_count: i64,
    pub uplay_games: CategoryUplayResponseItemsItemUplayGames,
    pub uplay_item_id: i64,
    pub uplay_last_activity: i64,
    pub uplay_psn_connected: i64,
    pub uplay_r6: bool,
    pub uplay_r6_ban: i64,
    pub uplay_r6_ban_active: bool,
    pub uplay_r6_external_warning: bool,
    pub uplay_r6_level: i64,
    pub uplay_r6_operators: String,
    pub uplay_r6_operators_count: i64,
    pub uplay_r6_rank: i64,
    pub uplay_r6_skins: String,
    pub uplay_r6_skins_count: i64,
    pub uplay_r6_steam_warning: bool,
    pub uplay_steam_connected: i64,
    pub uplay_subscription: String,
    pub uplay_subscription_end_date: i64,
    pub uplay_xbox_connected: i64,
    pub view_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponseItemsItemR6operatorsItem {
    pub img: String,
    pub name: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponseItemsItemUplayGames {
    #[serde(rename = "ffffffff-ffff-ffff-ffff-ffffffffffff")]
    pub ffffffff_ffff_ffff_ffff_ffffffffffff: CategoryUplayResponseItemsItemUplayGamesFfffffffFfffFfffFfffFfffffffffff,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponseItemsItemUplayGamesFfffffffFfffFfffFfffFfffffffffff {
    pub abbr: String,
    #[serde(rename = "gameId")]
    pub game_id: String,
    pub img: String,
    #[serde(rename = "pveTimePlayed")]
    pub pve_time_played: i64,
    #[serde(rename = "pvpTimePlayed")]
    pub pvp_time_played: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryVpnResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnResponseItemsItem {
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryVpnResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryVpnResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
    #[serde(rename = "vpnProductTitle")]
    pub vpn_product_title: String,
    pub vpn_expire_date: i64,
    pub vpn_item_id: i64,
    pub vpn_renewable: i64,
    pub vpn_service: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryWarfaceResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponseItemsItem {
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryWarfaceResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub domain: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryWarfaceResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
    pub wf_active_loan: i64,
    pub wf_bonus_rank: i64,
    pub wf_item_id: i64,
    pub wf_last_game_date: i64,
    pub wf_loan: bool,
    pub wf_mail_mobile: i64,
    pub wf_mobile: i64,
    pub wf_players: bool,
    pub wf_rank: i64,
    pub wf_server_1: i64,
    pub wf_server_2: i64,
    pub wf_server_3: i64,
    pub wf_servers: Vec<CategoryWarfaceResponseItemsItemWfServersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponseItemsItemWfServersItem {
    pub id: i64,
    pub rank: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryWotBlitzResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryWotBlitzResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryWotBlitzResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
    #[serde(rename = "wotLauncherTitle")]
    pub wot_launcher_title: String,
    #[serde(rename = "wotPremiumTankCount")]
    pub wot_premium_tank_count: i64,
    #[serde(rename = "wotPremiumTanks")]
    pub wot_premium_tanks: CategoryWotBlitzResponseItemsItemWotPremiumTanks,
    #[serde(rename = "wotRegionPhrase")]
    pub wot_region_phrase: String,
    #[serde(rename = "wotTankCount")]
    pub wot_tank_count: i64,
    #[serde(rename = "wotTanks")]
    pub wot_tanks: CategoryWotBlitzResponseItemsItemWotTanks,
    #[serde(rename = "wotTopPremiumTanks")]
    pub wot_top_premium_tanks: CategoryWotBlitzResponseItemsItemWotTopPremiumTanks,
    #[serde(rename = "wotTopTanks")]
    pub wot_top_tanks: CategoryWotBlitzResponseItemsItemWotTopTanks,
    pub wot_battle_count: i64,
    pub wot_blitz: i64,
    pub wot_credits: i64,
    pub wot_gold: i64,
    pub wot_has_clan: bool,
    pub wot_item_id: i64,
    pub wot_last_activity: i64,
    pub wot_loss_count: i64,
    pub wot_mobile: i64,
    pub wot_premium: i64,
    pub wot_premium_expires: i64,
    #[serde(rename = "wot_premium_tanks")]
    pub wot_premium_tanks_2: i64,
    pub wot_region: String,
    pub wot_register_date: i64,
    #[serde(rename = "wot_top_premium_tanks")]
    pub wot_top_premium_tanks_2: i64,
    #[serde(rename = "wot_top_tanks")]
    pub wot_top_tanks_2: i64,
    pub wot_win_count: i64,
    pub wot_win_count_percents: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks {
    #[serde(rename = "12545")]
    pub field_12545: CategoryWotBlitzResponseItemsItemWotPremiumTanks12545,
    #[serde(rename = "12673")]
    pub field_12673: CategoryWotBlitzResponseItemsItemWotPremiumTanks12673,
    #[serde(rename = "13345")]
    pub field_13345: CategoryWotBlitzResponseItemsItemWotPremiumTanks13345,
    #[serde(rename = "15697")]
    pub field_15697: CategoryWotBlitzResponseItemsItemWotPremiumTanks15697,
    #[serde(rename = "15953")]
    pub field_15953: CategoryWotBlitzResponseItemsItemWotPremiumTanks15953,
    #[serde(rename = "16193")]
    pub field_16193: CategoryWotBlitzResponseItemsItemWotPremiumTanks16193,
    #[serde(rename = "16705")]
    pub field_16705: CategoryWotBlitzResponseItemsItemWotPremiumTanks16705,
    #[serde(rename = "18241")]
    pub field_18241: CategoryWotBlitzResponseItemsItemWotPremiumTanks18241,
    #[serde(rename = "18513")]
    pub field_18513: CategoryWotBlitzResponseItemsItemWotPremiumTanks18513,
    #[serde(rename = "18753")]
    pub field_18753: CategoryWotBlitzResponseItemsItemWotPremiumTanks18753,
    #[serde(rename = "18769")]
    pub field_18769: CategoryWotBlitzResponseItemsItemWotPremiumTanks18769,
    #[serde(rename = "19025")]
    pub field_19025: CategoryWotBlitzResponseItemsItemWotPremiumTanks19025,
    #[serde(rename = "19489")]
    pub field_19489: CategoryWotBlitzResponseItemsItemWotPremiumTanks19489,
    #[serde(rename = "19713")]
    pub field_19713: CategoryWotBlitzResponseItemsItemWotPremiumTanks19713,
    #[serde(rename = "19985")]
    pub field_19985: CategoryWotBlitzResponseItemsItemWotPremiumTanks19985,
    #[serde(rename = "20305")]
    pub field_20305: CategoryWotBlitzResponseItemsItemWotPremiumTanks20305,
    #[serde(rename = "20481")]
    pub field_20481: CategoryWotBlitzResponseItemsItemWotPremiumTanks20481,
    #[serde(rename = "20513")]
    pub field_20513: CategoryWotBlitzResponseItemsItemWotPremiumTanks20513,
    #[serde(rename = "20737")]
    pub field_20737: CategoryWotBlitzResponseItemsItemWotPremiumTanks20737,
    #[serde(rename = "20817")]
    pub field_20817: CategoryWotBlitzResponseItemsItemWotPremiumTanks20817,
    #[serde(rename = "21025")]
    pub field_21025: CategoryWotBlitzResponseItemsItemWotPremiumTanks21025,
    #[serde(rename = "21265")]
    pub field_21265: CategoryWotBlitzResponseItemsItemWotPremiumTanks21265,
    #[serde(rename = "21329")]
    pub field_21329: CategoryWotBlitzResponseItemsItemWotPremiumTanks21329,
    #[serde(rename = "23057")]
    pub field_23057: CategoryWotBlitzResponseItemsItemWotPremiumTanks23057,
    #[serde(rename = "23297")]
    pub field_23297: CategoryWotBlitzResponseItemsItemWotPremiumTanks23297,
    #[serde(rename = "23313")]
    pub field_23313: CategoryWotBlitzResponseItemsItemWotPremiumTanks23313,
    #[serde(rename = "23825")]
    pub field_23825: CategoryWotBlitzResponseItemsItemWotPremiumTanks23825,
    #[serde(rename = "23841")]
    pub field_23841: CategoryWotBlitzResponseItemsItemWotPremiumTanks23841,
    #[serde(rename = "25361")]
    pub field_25361: CategoryWotBlitzResponseItemsItemWotPremiumTanks25361,
    #[serde(rename = "2609")]
    pub field_2609: CategoryWotBlitzResponseItemsItemWotPremiumTanks2609,
    #[serde(rename = "2849")]
    pub field_2849: CategoryWotBlitzResponseItemsItemWotPremiumTanks2849,
    #[serde(rename = "2945")]
    pub field_2945: CategoryWotBlitzResponseItemsItemWotPremiumTanks2945,
    #[serde(rename = "4881")]
    pub field_4881: CategoryWotBlitzResponseItemsItemWotPremiumTanks4881,
    #[serde(rename = "51473")]
    pub field_51473: CategoryWotBlitzResponseItemsItemWotPremiumTanks51473,
    #[serde(rename = "51729")]
    pub field_51729: CategoryWotBlitzResponseItemsItemWotPremiumTanks51729,
    #[serde(rename = "53025")]
    pub field_53025: CategoryWotBlitzResponseItemsItemWotPremiumTanks53025,
    #[serde(rename = "53761")]
    pub field_53761: CategoryWotBlitzResponseItemsItemWotPremiumTanks53761,
    #[serde(rename = "54545")]
    pub field_54545: CategoryWotBlitzResponseItemsItemWotPremiumTanks54545,
    #[serde(rename = "54785")]
    pub field_54785: CategoryWotBlitzResponseItemsItemWotPremiumTanks54785,
    #[serde(rename = "5489")]
    pub field_5489: CategoryWotBlitzResponseItemsItemWotPremiumTanks5489,
    #[serde(rename = "55297")]
    pub field_55297: CategoryWotBlitzResponseItemsItemWotPremiumTanks55297,
    #[serde(rename = "55889")]
    pub field_55889: CategoryWotBlitzResponseItemsItemWotPremiumTanks55889,
    #[serde(rename = "56097")]
    pub field_56097: CategoryWotBlitzResponseItemsItemWotPremiumTanks56097,
    #[serde(rename = "5681")]
    pub field_5681: CategoryWotBlitzResponseItemsItemWotPremiumTanks5681,
    #[serde(rename = "57105")]
    pub field_57105: CategoryWotBlitzResponseItemsItemWotPremiumTanks57105,
    #[serde(rename = "57361")]
    pub field_57361: CategoryWotBlitzResponseItemsItemWotPremiumTanks57361,
    #[serde(rename = "5745")]
    pub field_5745: CategoryWotBlitzResponseItemsItemWotPremiumTanks5745,
    #[serde(rename = "58881")]
    pub field_58881: CategoryWotBlitzResponseItemsItemWotPremiumTanks58881,
    #[serde(rename = "59137")]
    pub field_59137: CategoryWotBlitzResponseItemsItemWotPremiumTanks59137,
    #[serde(rename = "6001")]
    pub field_6001: CategoryWotBlitzResponseItemsItemWotPremiumTanks6001,
    #[serde(rename = "625")]
    pub field_625: CategoryWotBlitzResponseItemsItemWotPremiumTanks625,
    #[serde(rename = "6257")]
    pub field_6257: CategoryWotBlitzResponseItemsItemWotPremiumTanks6257,
    #[serde(rename = "62737")]
    pub field_62737: CategoryWotBlitzResponseItemsItemWotPremiumTanks62737,
    #[serde(rename = "64529")]
    pub field_64529: CategoryWotBlitzResponseItemsItemWotPremiumTanks64529,
    #[serde(rename = "65377")]
    pub field_65377: CategoryWotBlitzResponseItemsItemWotPremiumTanks65377,
    #[serde(rename = "6785")]
    pub field_6785: CategoryWotBlitzResponseItemsItemWotPremiumTanks6785,
    #[serde(rename = "7281")]
    pub field_7281: CategoryWotBlitzResponseItemsItemWotPremiumTanks7281,
    #[serde(rename = "7793")]
    pub field_7793: CategoryWotBlitzResponseItemsItemWotPremiumTanks7793,
    #[serde(rename = "8753")]
    pub field_8753: CategoryWotBlitzResponseItemsItemWotPremiumTanks8753,
    #[serde(rename = "9073")]
    pub field_9073: CategoryWotBlitzResponseItemsItemWotPremiumTanks9073,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks12545 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks12673 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks13345 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks15697 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks15953 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks16193 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks16705 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks18241 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks18513 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks18753 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks18769 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks19025 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks19489 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks19713 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks19985 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks20305 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks20481 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks20513 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks20737 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks20817 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks21025 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks21265 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks21329 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks23057 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks23297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks23313 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks23825 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks23841 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks25361 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks2609 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks2849 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks2945 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks4881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks51473 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks51729 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks53025 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks53761 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks54545 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks54785 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks5489 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks55297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks55889 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks56097 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks5681 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks57105 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks57361 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks5745 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks58881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks59137 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks6001 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks625 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks6257 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks62737 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks64529 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks65377 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks6785 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks7281 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks7793 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks8753 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotPremiumTanks9073 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks {
    #[serde(rename = "10241")]
    pub field_10241: CategoryWotBlitzResponseItemsItemWotTanks10241,
    #[serde(rename = "10273")]
    pub field_10273: CategoryWotBlitzResponseItemsItemWotTanks10273,
    #[serde(rename = "10289")]
    pub field_10289: CategoryWotBlitzResponseItemsItemWotTanks10289,
    #[serde(rename = "10369")]
    pub field_10369: CategoryWotBlitzResponseItemsItemWotTanks10369,
    #[serde(rename = "10785")]
    pub field_10785: CategoryWotBlitzResponseItemsItemWotTanks10785,
    #[serde(rename = "10881")]
    pub field_10881: CategoryWotBlitzResponseItemsItemWotTanks10881,
    #[serde(rename = "11553")]
    pub field_11553: CategoryWotBlitzResponseItemsItemWotTanks11553,
    #[serde(rename = "12049")]
    pub field_12049: CategoryWotBlitzResponseItemsItemWotTanks12049,
    #[serde(rename = "12305")]
    pub field_12305: CategoryWotBlitzResponseItemsItemWotTanks12305,
    #[serde(rename = "12545")]
    pub field_12545: CategoryWotBlitzResponseItemsItemWotTanks12545,
    #[serde(rename = "12673")]
    pub field_12673: CategoryWotBlitzResponseItemsItemWotTanks12673,
    #[serde(rename = "13089")]
    pub field_13089: CategoryWotBlitzResponseItemsItemWotTanks13089,
    #[serde(rename = "13185")]
    pub field_13185: CategoryWotBlitzResponseItemsItemWotTanks13185,
    #[serde(rename = "13345")]
    pub field_13345: CategoryWotBlitzResponseItemsItemWotTanks13345,
    #[serde(rename = "13569")]
    pub field_13569: CategoryWotBlitzResponseItemsItemWotTanks13569,
    #[serde(rename = "13825")]
    pub field_13825: CategoryWotBlitzResponseItemsItemWotTanks13825,
    #[serde(rename = "1409")]
    pub field_1409: CategoryWotBlitzResponseItemsItemWotTanks1409,
    #[serde(rename = "14337")]
    pub field_14337: CategoryWotBlitzResponseItemsItemWotTanks14337,
    #[serde(rename = "14609")]
    pub field_14609: CategoryWotBlitzResponseItemsItemWotTanks14609,
    #[serde(rename = "14881")]
    pub field_14881: CategoryWotBlitzResponseItemsItemWotTanks14881,
    #[serde(rename = "15697")]
    pub field_15697: CategoryWotBlitzResponseItemsItemWotTanks15697,
    #[serde(rename = "15953")]
    pub field_15953: CategoryWotBlitzResponseItemsItemWotTanks15953,
    #[serde(rename = "16193")]
    pub field_16193: CategoryWotBlitzResponseItemsItemWotTanks16193,
    #[serde(rename = "16401")]
    pub field_16401: CategoryWotBlitzResponseItemsItemWotTanks16401,
    #[serde(rename = "16705")]
    pub field_16705: CategoryWotBlitzResponseItemsItemWotTanks16705,
    #[serde(rename = "16897")]
    pub field_16897: CategoryWotBlitzResponseItemsItemWotTanks16897,
    #[serde(rename = "18001")]
    pub field_18001: CategoryWotBlitzResponseItemsItemWotTanks18001,
    #[serde(rename = "18209")]
    pub field_18209: CategoryWotBlitzResponseItemsItemWotTanks18209,
    #[serde(rename = "18241")]
    pub field_18241: CategoryWotBlitzResponseItemsItemWotTanks18241,
    #[serde(rename = "18513")]
    pub field_18513: CategoryWotBlitzResponseItemsItemWotTanks18513,
    #[serde(rename = "18753")]
    pub field_18753: CategoryWotBlitzResponseItemsItemWotTanks18753,
    #[serde(rename = "18769")]
    pub field_18769: CategoryWotBlitzResponseItemsItemWotTanks18769,
    #[serde(rename = "19025")]
    pub field_19025: CategoryWotBlitzResponseItemsItemWotTanks19025,
    #[serde(rename = "19217")]
    pub field_19217: CategoryWotBlitzResponseItemsItemWotTanks19217,
    #[serde(rename = "19489")]
    pub field_19489: CategoryWotBlitzResponseItemsItemWotTanks19489,
    #[serde(rename = "19537")]
    pub field_19537: CategoryWotBlitzResponseItemsItemWotTanks19537,
    #[serde(rename = "19713")]
    pub field_19713: CategoryWotBlitzResponseItemsItemWotTanks19713,
    #[serde(rename = "19985")]
    pub field_19985: CategoryWotBlitzResponseItemsItemWotTanks19985,
    #[serde(rename = "20001")]
    pub field_20001: CategoryWotBlitzResponseItemsItemWotTanks20001,
    #[serde(rename = "20257")]
    pub field_20257: CategoryWotBlitzResponseItemsItemWotTanks20257,
    #[serde(rename = "20305")]
    pub field_20305: CategoryWotBlitzResponseItemsItemWotTanks20305,
    #[serde(rename = "20481")]
    pub field_20481: CategoryWotBlitzResponseItemsItemWotTanks20481,
    #[serde(rename = "20513")]
    pub field_20513: CategoryWotBlitzResponseItemsItemWotTanks20513,
    #[serde(rename = "20737")]
    pub field_20737: CategoryWotBlitzResponseItemsItemWotTanks20737,
    #[serde(rename = "20817")]
    pub field_20817: CategoryWotBlitzResponseItemsItemWotTanks20817,
    #[serde(rename = "21025")]
    pub field_21025: CategoryWotBlitzResponseItemsItemWotTanks21025,
    #[serde(rename = "21265")]
    pub field_21265: CategoryWotBlitzResponseItemsItemWotTanks21265,
    #[serde(rename = "21329")]
    pub field_21329: CategoryWotBlitzResponseItemsItemWotTanks21329,
    #[serde(rename = "22817")]
    pub field_22817: CategoryWotBlitzResponseItemsItemWotTanks22817,
    #[serde(rename = "23057")]
    pub field_23057: CategoryWotBlitzResponseItemsItemWotTanks23057,
    #[serde(rename = "23297")]
    pub field_23297: CategoryWotBlitzResponseItemsItemWotTanks23297,
    #[serde(rename = "23313")]
    pub field_23313: CategoryWotBlitzResponseItemsItemWotTanks23313,
    #[serde(rename = "23825")]
    pub field_23825: CategoryWotBlitzResponseItemsItemWotTanks23825,
    #[serde(rename = "23841")]
    pub field_23841: CategoryWotBlitzResponseItemsItemWotTanks23841,
    #[serde(rename = "24321")]
    pub field_24321: CategoryWotBlitzResponseItemsItemWotTanks24321,
    #[serde(rename = "24849")]
    pub field_24849: CategoryWotBlitzResponseItemsItemWotTanks24849,
    #[serde(rename = "25361")]
    pub field_25361: CategoryWotBlitzResponseItemsItemWotTanks25361,
    #[serde(rename = "2609")]
    pub field_2609: CategoryWotBlitzResponseItemsItemWotTanks2609,
    #[serde(rename = "2625")]
    pub field_2625: CategoryWotBlitzResponseItemsItemWotTanks2625,
    #[serde(rename = "2849")]
    pub field_2849: CategoryWotBlitzResponseItemsItemWotTanks2849,
    #[serde(rename = "2945")]
    pub field_2945: CategoryWotBlitzResponseItemsItemWotTanks2945,
    #[serde(rename = "3121")]
    pub field_3121: CategoryWotBlitzResponseItemsItemWotTanks3121,
    #[serde(rename = "3649")]
    pub field_3649: CategoryWotBlitzResponseItemsItemWotTanks3649,
    #[serde(rename = "3681")]
    pub field_3681: CategoryWotBlitzResponseItemsItemWotTanks3681,
    #[serde(rename = "385")]
    pub field_385: CategoryWotBlitzResponseItemsItemWotTanks385,
    #[serde(rename = "3937")]
    pub field_3937: CategoryWotBlitzResponseItemsItemWotTanks3937,
    #[serde(rename = "4145")]
    pub field_4145: CategoryWotBlitzResponseItemsItemWotTanks4145,
    #[serde(rename = "4369")]
    pub field_4369: CategoryWotBlitzResponseItemsItemWotTanks4369,
    #[serde(rename = "4481")]
    pub field_4481: CategoryWotBlitzResponseItemsItemWotTanks4481,
    #[serde(rename = "4881")]
    pub field_4881: CategoryWotBlitzResponseItemsItemWotTanks4881,
    #[serde(rename = "5137")]
    pub field_5137: CategoryWotBlitzResponseItemsItemWotTanks5137,
    #[serde(rename = "51473")]
    pub field_51473: CategoryWotBlitzResponseItemsItemWotTanks51473,
    #[serde(rename = "51729")]
    pub field_51729: CategoryWotBlitzResponseItemsItemWotTanks51729,
    #[serde(rename = "53025")]
    pub field_53025: CategoryWotBlitzResponseItemsItemWotTanks53025,
    #[serde(rename = "53761")]
    pub field_53761: CategoryWotBlitzResponseItemsItemWotTanks53761,
    #[serde(rename = "5393")]
    pub field_5393: CategoryWotBlitzResponseItemsItemWotTanks5393,
    #[serde(rename = "5425")]
    pub field_5425: CategoryWotBlitzResponseItemsItemWotTanks5425,
    #[serde(rename = "54545")]
    pub field_54545: CategoryWotBlitzResponseItemsItemWotTanks54545,
    #[serde(rename = "54785")]
    pub field_54785: CategoryWotBlitzResponseItemsItemWotTanks54785,
    #[serde(rename = "5489")]
    pub field_5489: CategoryWotBlitzResponseItemsItemWotTanks5489,
    #[serde(rename = "5505")]
    pub field_5505: CategoryWotBlitzResponseItemsItemWotTanks5505,
    #[serde(rename = "55297")]
    pub field_55297: CategoryWotBlitzResponseItemsItemWotTanks55297,
    #[serde(rename = "55889")]
    pub field_55889: CategoryWotBlitzResponseItemsItemWotTanks55889,
    #[serde(rename = "56097")]
    pub field_56097: CategoryWotBlitzResponseItemsItemWotTanks56097,
    #[serde(rename = "5681")]
    pub field_5681: CategoryWotBlitzResponseItemsItemWotTanks5681,
    #[serde(rename = "57105")]
    pub field_57105: CategoryWotBlitzResponseItemsItemWotTanks57105,
    #[serde(rename = "57361")]
    pub field_57361: CategoryWotBlitzResponseItemsItemWotTanks57361,
    #[serde(rename = "5745")]
    pub field_5745: CategoryWotBlitzResponseItemsItemWotTanks5745,
    #[serde(rename = "58641")]
    pub field_58641: CategoryWotBlitzResponseItemsItemWotTanks58641,
    #[serde(rename = "58881")]
    pub field_58881: CategoryWotBlitzResponseItemsItemWotTanks58881,
    #[serde(rename = "59137")]
    pub field_59137: CategoryWotBlitzResponseItemsItemWotTanks59137,
    #[serde(rename = "6001")]
    pub field_6001: CategoryWotBlitzResponseItemsItemWotTanks6001,
    #[serde(rename = "6145")]
    pub field_6145: CategoryWotBlitzResponseItemsItemWotTanks6145,
    #[serde(rename = "6209")]
    pub field_6209: CategoryWotBlitzResponseItemsItemWotTanks6209,
    #[serde(rename = "625")]
    pub field_625: CategoryWotBlitzResponseItemsItemWotTanks625,
    #[serde(rename = "6257")]
    pub field_6257: CategoryWotBlitzResponseItemsItemWotTanks6257,
    #[serde(rename = "62737")]
    pub field_62737: CategoryWotBlitzResponseItemsItemWotTanks62737,
    #[serde(rename = "641")]
    pub field_641: CategoryWotBlitzResponseItemsItemWotTanks641,
    #[serde(rename = "6449")]
    pub field_6449: CategoryWotBlitzResponseItemsItemWotTanks6449,
    #[serde(rename = "64529")]
    pub field_64529: CategoryWotBlitzResponseItemsItemWotTanks64529,
    #[serde(rename = "65377")]
    pub field_65377: CategoryWotBlitzResponseItemsItemWotTanks65377,
    #[serde(rename = "6753")]
    pub field_6753: CategoryWotBlitzResponseItemsItemWotTanks6753,
    #[serde(rename = "6785")]
    pub field_6785: CategoryWotBlitzResponseItemsItemWotTanks6785,
    #[serde(rename = "6929")]
    pub field_6929: CategoryWotBlitzResponseItemsItemWotTanks6929,
    #[serde(rename = "6993")]
    pub field_6993: CategoryWotBlitzResponseItemsItemWotTanks6993,
    #[serde(rename = "7169")]
    pub field_7169: CategoryWotBlitzResponseItemsItemWotTanks7169,
    #[serde(rename = "7249")]
    pub field_7249: CategoryWotBlitzResponseItemsItemWotTanks7249,
    #[serde(rename = "7281")]
    pub field_7281: CategoryWotBlitzResponseItemsItemWotTanks7281,
    #[serde(rename = "7297")]
    pub field_7297: CategoryWotBlitzResponseItemsItemWotTanks7297,
    #[serde(rename = "7793")]
    pub field_7793: CategoryWotBlitzResponseItemsItemWotTanks7793,
    #[serde(rename = "7953")]
    pub field_7953: CategoryWotBlitzResponseItemsItemWotTanks7953,
    #[serde(rename = "8753")]
    pub field_8753: CategoryWotBlitzResponseItemsItemWotTanks8753,
    #[serde(rename = "9073")]
    pub field_9073: CategoryWotBlitzResponseItemsItemWotTanks9073,
    #[serde(rename = "9297")]
    pub field_9297: CategoryWotBlitzResponseItemsItemWotTanks9297,
    #[serde(rename = "9489")]
    pub field_9489: CategoryWotBlitzResponseItemsItemWotTanks9489,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks10241 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks10273 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks10289 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks10369 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks10785 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks10881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks11553 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks12049 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks12305 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks12545 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks12673 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks13089 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks13185 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks13345 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks13569 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks13825 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks1409 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks14337 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks14609 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks14881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks15697 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks15953 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks16193 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks16401 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks16705 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks16897 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks18001 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks18209 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks18241 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks18513 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks18753 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks18769 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks19025 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks19217 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks19489 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks19537 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks19713 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks19985 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20001 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20257 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20305 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20481 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20513 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20737 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks20817 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks21025 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks21265 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks21329 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks22817 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks23057 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks23297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks23313 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks23825 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks23841 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks24321 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks24849 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks25361 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks2609 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks2625 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks2849 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks2945 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks3121 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks3649 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks3681 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks385 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks3937 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks4145 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks4369 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks4481 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks4881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5137 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks51473 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks51729 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks53025 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks53761 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5393 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5425 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks54545 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks54785 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5489 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5505 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks55297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks55889 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks56097 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5681 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks57105 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks57361 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks5745 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks58641 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks58881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks59137 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6001 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6145 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6209 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks625 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6257 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks62737 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks641 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6449 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks64529 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks65377 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6753 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6785 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6929 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks6993 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks7169 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks7249 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks7281 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks7297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks7793 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks7953 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks8753 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks9073 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks9297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTanks9489 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopPremiumTanks {
    #[serde(rename = "15697")]
    pub field_15697: CategoryWotBlitzResponseItemsItemWotTopPremiumTanks15697,
    #[serde(rename = "23313")]
    pub field_23313: CategoryWotBlitzResponseItemsItemWotTopPremiumTanks23313,
    #[serde(rename = "5681")]
    pub field_5681: CategoryWotBlitzResponseItemsItemWotTopPremiumTanks5681,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopPremiumTanks15697 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopPremiumTanks23313 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopPremiumTanks5681 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks {
    #[serde(rename = "10289")]
    pub field_10289: CategoryWotBlitzResponseItemsItemWotTopTanks10289,
    #[serde(rename = "10369")]
    pub field_10369: CategoryWotBlitzResponseItemsItemWotTopTanks10369,
    #[serde(rename = "10785")]
    pub field_10785: CategoryWotBlitzResponseItemsItemWotTopTanks10785,
    #[serde(rename = "12049")]
    pub field_12049: CategoryWotBlitzResponseItemsItemWotTopTanks12049,
    #[serde(rename = "12305")]
    pub field_12305: CategoryWotBlitzResponseItemsItemWotTopTanks12305,
    #[serde(rename = "13089")]
    pub field_13089: CategoryWotBlitzResponseItemsItemWotTopTanks13089,
    #[serde(rename = "13185")]
    pub field_13185: CategoryWotBlitzResponseItemsItemWotTopTanks13185,
    #[serde(rename = "13569")]
    pub field_13569: CategoryWotBlitzResponseItemsItemWotTopTanks13569,
    #[serde(rename = "13825")]
    pub field_13825: CategoryWotBlitzResponseItemsItemWotTopTanks13825,
    #[serde(rename = "14337")]
    pub field_14337: CategoryWotBlitzResponseItemsItemWotTopTanks14337,
    #[serde(rename = "14609")]
    pub field_14609: CategoryWotBlitzResponseItemsItemWotTopTanks14609,
    #[serde(rename = "14881")]
    pub field_14881: CategoryWotBlitzResponseItemsItemWotTopTanks14881,
    #[serde(rename = "15697")]
    pub field_15697: CategoryWotBlitzResponseItemsItemWotTopTanks15697,
    #[serde(rename = "16897")]
    pub field_16897: CategoryWotBlitzResponseItemsItemWotTopTanks16897,
    #[serde(rename = "18001")]
    pub field_18001: CategoryWotBlitzResponseItemsItemWotTopTanks18001,
    #[serde(rename = "19217")]
    pub field_19217: CategoryWotBlitzResponseItemsItemWotTopTanks19217,
    #[serde(rename = "19537")]
    pub field_19537: CategoryWotBlitzResponseItemsItemWotTopTanks19537,
    #[serde(rename = "20257")]
    pub field_20257: CategoryWotBlitzResponseItemsItemWotTopTanks20257,
    #[serde(rename = "22817")]
    pub field_22817: CategoryWotBlitzResponseItemsItemWotTopTanks22817,
    #[serde(rename = "23313")]
    pub field_23313: CategoryWotBlitzResponseItemsItemWotTopTanks23313,
    #[serde(rename = "24321")]
    pub field_24321: CategoryWotBlitzResponseItemsItemWotTopTanks24321,
    #[serde(rename = "3649")]
    pub field_3649: CategoryWotBlitzResponseItemsItemWotTopTanks3649,
    #[serde(rename = "3681")]
    pub field_3681: CategoryWotBlitzResponseItemsItemWotTopTanks3681,
    #[serde(rename = "385")]
    pub field_385: CategoryWotBlitzResponseItemsItemWotTopTanks385,
    #[serde(rename = "3937")]
    pub field_3937: CategoryWotBlitzResponseItemsItemWotTopTanks3937,
    #[serde(rename = "4145")]
    pub field_4145: CategoryWotBlitzResponseItemsItemWotTopTanks4145,
    #[serde(rename = "4481")]
    pub field_4481: CategoryWotBlitzResponseItemsItemWotTopTanks4481,
    #[serde(rename = "5425")]
    pub field_5425: CategoryWotBlitzResponseItemsItemWotTopTanks5425,
    #[serde(rename = "5505")]
    pub field_5505: CategoryWotBlitzResponseItemsItemWotTopTanks5505,
    #[serde(rename = "5681")]
    pub field_5681: CategoryWotBlitzResponseItemsItemWotTopTanks5681,
    #[serde(rename = "58641")]
    pub field_58641: CategoryWotBlitzResponseItemsItemWotTopTanks58641,
    #[serde(rename = "6145")]
    pub field_6145: CategoryWotBlitzResponseItemsItemWotTopTanks6145,
    #[serde(rename = "6209")]
    pub field_6209: CategoryWotBlitzResponseItemsItemWotTopTanks6209,
    #[serde(rename = "6449")]
    pub field_6449: CategoryWotBlitzResponseItemsItemWotTopTanks6449,
    #[serde(rename = "6753")]
    pub field_6753: CategoryWotBlitzResponseItemsItemWotTopTanks6753,
    #[serde(rename = "6929")]
    pub field_6929: CategoryWotBlitzResponseItemsItemWotTopTanks6929,
    #[serde(rename = "7169")]
    pub field_7169: CategoryWotBlitzResponseItemsItemWotTopTanks7169,
    #[serde(rename = "7249")]
    pub field_7249: CategoryWotBlitzResponseItemsItemWotTopTanks7249,
    #[serde(rename = "7297")]
    pub field_7297: CategoryWotBlitzResponseItemsItemWotTopTanks7297,
    #[serde(rename = "9297")]
    pub field_9297: CategoryWotBlitzResponseItemsItemWotTopTanks9297,
    #[serde(rename = "9489")]
    pub field_9489: CategoryWotBlitzResponseItemsItemWotTopTanks9489,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks10289 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks10369 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks10785 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks12049 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks12305 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks13089 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks13185 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks13569 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks13825 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks14337 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks14609 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks14881 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks15697 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks16897 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks18001 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks19217 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks19537 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks20257 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks22817 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks23313 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks24321 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks3649 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks3681 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks385 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks3937 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks4145 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks4481 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks5425 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks5505 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks5681 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks58641 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks6145 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks6209 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks6449 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks6753 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks6929 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks7169 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks7249 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks7297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks9297 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponseItemsItemWotTopTanks9489 {
    pub image_url: String,
    pub is_collectible: i64,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<CategoryWotResponseItemsItem>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItem {
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: CategoryWotResponseItemsItemBumpSettings,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub category_id: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: serde_json::Value,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub feedback_data: String,
    pub guarantee: serde_json::Value,
    #[serde(rename = "hasPendingAutoBuy")]
    pub has_pending_auto_buy: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub note_text: serde_json::Value,
    pub nsb: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: i64,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub restore_items_category_count: i64,
    pub rub_price: i64,
    pub seller: CategoryWotResponseItemsItemSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub sold_items_category_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub title: String,
    pub title_en: String,
    pub update_stat_date: i64,
    pub view_count: i64,
    #[serde(rename = "wotLauncherTitle")]
    pub wot_launcher_title: String,
    #[serde(rename = "wotPremiumTankCount")]
    pub wot_premium_tank_count: i64,
    #[serde(rename = "wotPremiumTanks")]
    pub wot_premium_tanks: CategoryWotResponseItemsItemWotPremiumTanks,
    #[serde(rename = "wotRegionPhrase")]
    pub wot_region_phrase: String,
    #[serde(rename = "wotTankCount")]
    pub wot_tank_count: i64,
    #[serde(rename = "wotTanks")]
    pub wot_tanks: CategoryWotResponseItemsItemWotTanks,
    #[serde(rename = "wotTopPremiumTanks")]
    pub wot_top_premium_tanks: CategoryWotResponseItemsItemWotTopPremiumTanks,
    #[serde(rename = "wotTopTanks")]
    pub wot_top_tanks: CategoryWotResponseItemsItemWotTopTanks,
    pub wot_battle_count: i64,
    pub wot_blitz: i64,
    pub wot_credits: i64,
    pub wot_gold: i64,
    pub wot_has_clan: bool,
    pub wot_item_id: i64,
    pub wot_last_activity: i64,
    pub wot_loss_count: i64,
    pub wot_mobile: i64,
    pub wot_premium: i64,
    pub wot_premium_expires: i64,
    #[serde(rename = "wot_premium_tanks")]
    pub wot_premium_tanks_2: i64,
    pub wot_region: String,
    pub wot_register_date: i64,
    #[serde(rename = "wot_top_premium_tanks")]
    pub wot_top_premium_tanks_2: i64,
    #[serde(rename = "wot_top_tanks")]
    pub wot_top_tanks_2: i64,
    pub wot_win_count: i64,
    pub wot_win_count_percents: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub restore_data: String,
    pub restore_percents: i64,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks {
    #[serde(rename = "30465")]
    pub field_30465: CategoryWotResponseItemsItemWotPremiumTanks30465,
    #[serde(rename = "43841")]
    pub field_43841: CategoryWotResponseItemsItemWotPremiumTanks43841,
    #[serde(rename = "46097")]
    pub field_46097: CategoryWotResponseItemsItemWotPremiumTanks46097,
    #[serde(rename = "47873")]
    pub field_47873: CategoryWotResponseItemsItemWotPremiumTanks47873,
    #[serde(rename = "50977")]
    pub field_50977: CategoryWotResponseItemsItemWotPremiumTanks50977,
    #[serde(rename = "51233")]
    pub field_51233: CategoryWotResponseItemsItemWotPremiumTanks51233,
    #[serde(rename = "51585")]
    pub field_51585: CategoryWotResponseItemsItemWotPremiumTanks51585,
    #[serde(rename = "55569")]
    pub field_55569: CategoryWotResponseItemsItemWotPremiumTanks55569,
    #[serde(rename = "57377")]
    pub field_57377: CategoryWotResponseItemsItemWotPremiumTanks57377,
    #[serde(rename = "60945")]
    pub field_60945: CategoryWotResponseItemsItemWotPremiumTanks60945,
    #[serde(rename = "62497")]
    pub field_62497: CategoryWotResponseItemsItemWotPremiumTanks62497,
    #[serde(rename = "7937025")]
    pub field_7937025: CategoryWotResponseItemsItemWotPremiumTanks7937025,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks30465 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks43841 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks46097 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks47873 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks50977 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks51233 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks51585 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks55569 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks57377 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks60945 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks62497 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotPremiumTanks7937025 {
    pub image_url: String,
    pub is_premium: i64,
    pub name: String,
    pub short_name: String,
    pub tank_id: i64,
    pub tier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotTanks {
    #[serde(rename = "00000")]
    pub field_00000: CategoryWotResponseItemsItemWotTanks00000,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotTanks00000 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotTopPremiumTanks {
    #[serde(rename = "00000")]
    pub field_00000: CategoryWotResponseItemsItemWotTopPremiumTanks00000,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotTopPremiumTanks00000 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotTopTanks {
    #[serde(rename = "00000")]
    pub field_00000: CategoryWotResponseItemsItemWotTopTanks00000,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponseItemsItemWotTopTanks00000 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsCreateRequest {
    pub category_id: MarketCategoryIdmodel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    pub discount_percent: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
    pub min_price: f64,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsCreateResponse {
    pub discount: MarketDiscountModel,
    pub system_info: MarketRespSystemInfo,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsDeleteRequest {
    pub discount_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsEditRequest {
    pub discount_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsEditResponse {
    pub discounts: Vec<MarketDiscountModel>,
    pub system_info: MarketRespSystemInfo,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsGetResponse {
    pub discounts: Vec<MarketDiscountModel>,
    pub system_info: MarketRespSystemInfo,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ImapCreateRequest {
    pub domain: String,
    pub imap_server: String,
    pub port: i64,
    pub secure: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ImapDeleteRequest {
    pub domain: String,
}


pub type ListFavoritesResponse = MarketItemListModel;

pub type ListOrdersResponse = MarketItemListModel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponse {
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "userItemStates")]
    pub user_item_states: ListStatesResponseUserItemStates,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStates {
    pub active: ListStatesResponseUserItemStatesActive,
    pub auto_bump: ListStatesResponseUserItemStatesAutoBump,
    pub awaiting: ListStatesResponseUserItemStatesAwaiting,
    pub closed: ListStatesResponseUserItemStatesClosed,
    pub closed_inactive: ListStatesResponseUserItemStatesClosedInactive,
    pub deleted: ListStatesResponseUserItemStatesDeleted,
    pub discount_request: ListStatesResponseUserItemStatesDiscountRequest,
    pub in_buyers_favorites: ListStatesResponseUserItemStatesInBuyersFavorites,
    pub paid: ListStatesResponseUserItemStatesPaid,
    pub pending_deletion: ListStatesResponseUserItemStatesPendingDeletion,
    pub pre_active: ListStatesResponseUserItemStatesPreActive,
    pub pre_upload: ListStatesResponseUserItemStatesPreUpload,
    pub stickied: ListStatesResponseUserItemStatesStickied,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesActive {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesAutoBump {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesAwaiting {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesClosed {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesClosedInactive {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesDeleted {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesDiscountRequest {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesInBuyersFavorites {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesPaid {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesPendingDeletion {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesPreActive {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesPreUpload {
    pub item_count: i64,
    pub item_state: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponseUserItemStatesStickied {
    pub item_count: i64,
    pub item_state: String,
    #[serde(rename = "stickyLimit")]
    pub sticky_limit: i64,
    pub title: String,
}


pub type ListUserResponse = MarketItemListModel;

pub type ListViewedResponse = MarketItemListModel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAipriceResponse {
    pub price: i64,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAutoBumpRequest {
    pub hour: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAutoBuyPriceResponse {
    pub price: i64,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingBulkGetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<Vec<MarketItemIdmodel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingBulkGetResponse {
    pub items: Vec<ManagingBulkGetResponseItemsItem>,
    pub left_item_id: Vec<i64>,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingBulkGetResponseItemsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "0")]
    pub field_0: Option<MarketItemModel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingChangePasswordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_cancel")]
    pub cancel: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingChangePasswordResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub new_password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCheckGuaranteeResponse {
    pub message: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimRequest {
    pub item_id: MarketItemIdmodel,
    pub post_body: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponse {
    pub system_info: ManagingCreateClaimResponseSystemInfo,
    pub thread: ManagingCreateClaimResponseThread,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseSystemInfo {
    pub time: i64,
    pub visitor_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThread {
    pub creator_user_id: i64,
    pub creator_username: String,
    pub first_post: ManagingCreateClaimResponseThreadFirstPost,
    pub forum: ManagingCreateClaimResponseThreadForum,
    pub forum_id: i64,
    pub links: ManagingCreateClaimResponseThreadLinks,
    pub permissions: ManagingCreateClaimResponseThreadPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadFirstPost {
    pub like_users: Vec<ManagingCreateClaimResponseThreadFirstPostLikeUsersItem>,
    pub links: ManagingCreateClaimResponseThreadFirstPostLinks,
    pub permissions: ManagingCreateClaimResponseThreadFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadFirstPostLikeUsersItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_style_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniq_username_css: Option<String>,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<ManagingCreateClaimResponseThreadForumForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ManagingCreateClaimResponseThreadForumLinks,
    pub permissions: ManagingCreateClaimResponseThreadForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadForumForumPrefixesItem {
    pub group_prefixes: Vec<ManagingCreateClaimResponseThreadForumForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadForumForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub last_poster: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponseThreadPermissions {
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingDeclineVideoRecordingRequest {
    pub i_voluntarily_and_with_full_awareness_of_my_actions_waive_any_claims_regarding_this_item: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}


pub type ManagingEmailCodeResponse = MarketConfirmationCodeModel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetLetters2Response {
    pub email: String,
    pub letters: Vec<ManagingGetLetters2ResponseLettersItem>,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetLetters2ResponseLettersItem {
    pub date: i64,
    pub from: String,
    #[serde(rename = "textHtml")]
    pub text_html: String,
    #[serde(rename = "textPlain")]
    pub text_plain: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetResponse {
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canCancelConfirmedBuy")]
    pub can_cancel_confirmed_buy: bool,
    #[serde(rename = "canChangeOwner")]
    pub can_change_owner: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canViewItemHistory")]
    pub can_view_item_history: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    #[serde(rename = "cannotBuyItemError")]
    pub cannot_buy_item_error: String,
    #[serde(rename = "faveCount")]
    pub fave_count: bool,
    #[serde(rename = "isVisibleItem")]
    pub is_visible_item: bool,
    pub item: MarketItemModel,
    #[serde(rename = "itemLink")]
    pub item_link: String,
    #[serde(rename = "sameItemsCount")]
    pub same_items_count: i64,
    #[serde(rename = "sameItemsIds")]
    pub same_items_ids: Vec<i64>,
    #[serde(rename = "showToFavouritesButton")]
    pub show_to_favourites_button: bool,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingImageResponse {
    pub base64: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingNoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingPublicTagRequest {
    pub tag_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingPublicUntagRequest {
    pub tag_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamGetMafileResponse {
    #[serde(rename = "maFile")]
    pub ma_file: ManagingSteamGetMafileResponseMaFile,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamGetMafileResponseMaFile {
    #[serde(rename = "Session")]
    pub session: ManagingSteamGetMafileResponseMaFileSession,
    pub account_name: String,
    pub device_id: String,
    pub fully_enrolled: bool,
    pub identity_secret: String,
    pub revocation_code: String,
    pub secret_1: String,
    pub serial_number: i64,
    pub shared_secret: String,
    pub token_gid: String,
    pub uri: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamGetMafileResponseMaFileSession {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "SessionID")]
    pub session_id: String,
    #[serde(rename = "SteamID")]
    pub steam_id: String,
    #[serde(rename = "SteamLoginSecure")]
    pub steam_login_secure: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "appId")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ManagingSteamInventoryValueResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<MarketRespSystemInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponseData {
    #[serde(rename = "appId")]
    pub app_id: i64,
    #[serde(rename = "appTitle")]
    pub app_title: String,
    pub currency: String,
    #[serde(rename = "currencyIcon")]
    pub currency_icon: String,
    #[serde(rename = "itemCount")]
    pub item_count: i64,
    pub items: ManagingSteamInventoryValueResponseDataItems,
    pub language: String,
    #[serde(rename = "marketableItemCount")]
    pub marketable_item_count: i64,
    pub steam_id: String,
    pub time: i64,
    #[serde(rename = "totalValue")]
    pub total_value: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponseDataItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "0")]
    pub field_0: Option<ManagingSteamInventoryValueResponseDataItems0>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponseDataItems0 {
    pub classid: String,
    pub count: i64,
    pub fraudwarnings: serde_json::Value,
    pub image_url: String,
    pub market_hash_name: String,
    pub marketable: i64,
    pub price: f64,
    pub stickers: ManagingSteamInventoryValueResponseDataItems0Stickers,
    pub title: String,
    pub tradable: i64,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponseDataItems0Stickers {
    pub count: i64,
    pub images: Vec<String>,
    #[serde(rename = "stickerCount")]
    pub sticker_count: i64,
    pub title: String,
}


pub type ManagingSteamMafileCodeResponse = MarketConfirmationCodeModel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamSdaRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamUpdateValueRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamUpdateValueResponse {
    pub item: MarketItemModel,
    pub status: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "appId")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ManagingSteamValueResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<MarketRespSystemInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponseData {
    #[serde(rename = "appId")]
    pub app_id: i64,
    #[serde(rename = "appTitle")]
    pub app_title: String,
    pub currency: String,
    #[serde(rename = "currencyIcon")]
    pub currency_icon: String,
    #[serde(rename = "itemCount")]
    pub item_count: i64,
    pub items: ManagingSteamValueResponseDataItems,
    pub language: String,
    #[serde(rename = "marketableItemCount")]
    pub marketable_item_count: i64,
    pub steam_id: String,
    pub time: i64,
    #[serde(rename = "totalValue")]
    pub total_value: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponseDataItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "0")]
    pub field_0: Option<ManagingSteamValueResponseDataItems0>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponseDataItems0 {
    pub classid: String,
    pub count: i64,
    pub fraudwarnings: serde_json::Value,
    pub image_url: String,
    pub market_hash_name: String,
    pub marketable: i64,
    pub price: f64,
    pub stickers: ManagingSteamValueResponseDataItems0Stickers,
    pub title: String,
    pub tradable: i64,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponseDataItems0Stickers {
    pub count: i64,
    pub images: Vec<String>,
    #[serde(rename = "stickerCount")]
    pub sticker_count: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTagRequest {
    pub tag_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTelegramCodeResponse {
    pub codes: ManagingTelegramCodeResponseCodes,
    pub item: MarketItemModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTelegramCodeResponseCodes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTempEmailPasswordResponse {
    pub item: ManagingTempEmailPasswordResponseItem,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTempEmailPasswordResponseItem {
    pub account: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTransferRequest {
    pub secret_answer: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingUntagRequest {
    pub tag_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MangingDeleteRequest {
    pub reason: String,
}


pub type MarketCategoryIdmodel = i64;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketConfirmationCodeModel {
    #[serde(rename = "codeData")]
    pub code_data: MarketConfirmationCodeModelCodeData,
    pub item: MarketItemModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketConfirmationCodeModelCodeData {
    pub code: String,
    pub date: i64,
    #[serde(rename = "textPlain")]
    pub text_plain: String,
}


pub type MarketCurrencyModel = String;

pub type MarketDatePeriodModel = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketDiscountModel {
    pub category_id: i64,
    pub discount_id: i64,
    pub discount_percent: i64,
    pub discount_user_id: i64,
    pub max_price: i64,
    pub min_price: i64,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketExtraModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark_ascended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "checkChannels")]
    pub check_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "checkHypixelBan")]
    pub check_hypixel_ban: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "checkSpam")]
    pub check_spam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "confirmationCode")]
    pub confirmation_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_login: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dota2_mmr: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ea_games: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_currency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_currency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_without_cookies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "telegramClient")]
    pub telegram_client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "telegramJson")]
    pub telegram_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub the_quarry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplay_games: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warframe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_currency: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketInvoiceModel {
    pub additional_data: String,
    pub amount: i64,
    pub comment: String,
    pub expires_at: i64,
    pub invoice_date: i64,
    pub invoice_id: i64,
    pub is_test: bool,
    pub merchant_id: i64,
    pub paid_date: i64,
    pub payer_user_id: i64,
    pub payment_id: String,
    pub resend_attempts: i64,
    pub status: String,
    pub url: String,
    pub url_callback: String,
    pub url_success: String,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemFromListModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bumpSettings")]
    pub bump_settings: Option<MarketItemFromListModelBumpSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canCloseItem")]
    pub can_close_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canEditItem")]
    pub can_edit_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canOpenItem")]
    pub can_open_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canStickItem")]
    pub can_stick_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_guarantee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isIgnored")]
    pub is_ignored: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sticky: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resale_item_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rub_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<MarketItemFromListModelSeller>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_stat_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemFromListModelBumpSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorPhrase")]
    pub error_phrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemFromListModelSeller {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_item_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_style_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_percents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_items_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


pub type MarketItemIdmodel = i64;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemListModel {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<MarketItemFromListModel>,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<MarketItemFromListModel>,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModel {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<MarketItemModelAccountLinksItem>,
    pub account_last_activity: i64,
    #[serde(rename = "aiPrice")]
    pub ai_price: i64,
    #[serde(rename = "aiPriceCheckDate")]
    pub ai_price_check_date: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "autoBuyPrice")]
    pub auto_buy_price: i64,
    #[serde(rename = "autoBuyPriceCheckDate")]
    pub auto_buy_price_check_date: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: MarketItemModelBumpSettings,
    pub buyer: MarketItemModelBuyer,
    pub buyer_avatar_date: i64,
    pub buyer_display_icon_group_id: i64,
    pub buyer_uniq_banner: String,
    pub buyer_user_group_id: i64,
    #[serde(rename = "canAskDiscount")]
    pub can_ask_discount: bool,
    #[serde(rename = "canChangeEmailPassword")]
    pub can_change_email_password: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCheckAiPrice")]
    pub can_check_ai_price: bool,
    #[serde(rename = "canCheckAutoBuyPrice")]
    pub can_check_auto_buy_price: bool,
    #[serde(rename = "canCheckGuarantee")]
    pub can_check_guarantee: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItem")]
    pub can_resell_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canShareItem")]
    pub can_share_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewAccountLoginAndTempEmail")]
    pub can_view_account_login_and_temp_email: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewItemViews")]
    pub can_view_item_views: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub cart_price: serde_json::Value,
    pub category_id: i64,
    pub content_id: serde_json::Value,
    pub content_type: serde_json::Value,
    #[serde(rename = "copyFormatData")]
    pub copy_format_data: MarketItemModelCopyFormatData,
    #[serde(rename = "customFields")]
    pub custom_fields: MarketItemModelCustomFields,
    pub delete_date: i64,
    pub delete_reason: String,
    pub delete_user_id: i64,
    pub delete_username: String,
    pub deposit: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    #[serde(rename = "externalAuth")]
    pub external_auth: Vec<serde_json::Value>,
    #[serde(rename = "extraPrices")]
    pub extra_prices: Vec<MarketItemModelExtraPricesItem>,
    pub feedback_data: String,
    #[serde(rename = "getEmailCodeDisplayLogin")]
    pub get_email_code_display_login: serde_json::Value,
    pub guarantee: MarketItemModelGuarantee,
    #[serde(rename = "imagePreviewLinks")]
    pub image_preview_links: Vec<String>,
    pub in_cart: serde_json::Value,
    pub information: String,
    pub information_en: String,
    #[serde(rename = "isBirthdayToday")]
    pub is_birthday_today: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isPersonalAccount")]
    pub is_personal_account: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    #[serde(rename = "isTrusted")]
    pub is_trusted: bool,
    pub is_fave: serde_json::Value,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub login: String,
    #[serde(rename = "loginData")]
    pub login_data: MarketItemModelLoginData,
    pub market_custom_title: String,
    pub max_discount_percent: i64,
    #[serde(rename = "needToRequireVideoToViewLoginData")]
    pub need_to_require_video_to_view_login_data: bool,
    pub note_text: String,
    pub nsb: i64,
    pub pending_deletion_date: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: f64,
    #[serde(rename = "priceWithSellerFeeLabel")]
    pub price_with_seller_fee_label: String,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: MarketItemModelSeller,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: MarketItemModelTags,
    pub temp_email: String,
    pub title: String,
    pub title_en: String,
    #[serde(rename = "uniqueKeyExists")]
    pub unique_key_exists: bool,
    pub update_stat_date: i64,
    pub user_allow_ask_discount: i64,
    pub view_count: i64,
    #[serde(rename = "visitorIsAuthor")]
    pub visitor_is_author: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelAccountLinksItem {
    #[serde(rename = "iconClass")]
    pub icon_class: String,
    pub link: String,
    pub text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelBumpSettings {
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: bool,
    #[serde(rename = "canBumpItemGlobally")]
    pub can_bump_item_globally: bool,
    #[serde(rename = "errorPhrase")]
    pub error_phrase: serde_json::Value,
    #[serde(rename = "nextAllowedBumpDate")]
    pub next_allowed_bump_date: serde_json::Value,
    #[serde(rename = "shortErrorPhrase")]
    pub short_error_phrase: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelBuyer {
    pub display_icon_group_id: i64,
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub operation_date: i64,
    pub uniq_banner: String,
    pub uniq_username_css: String,
    pub user_group_id: i64,
    pub user_id: i64,
    pub username: String,
    #[serde(rename = "visitorIsBuyer")]
    pub visitor_is_buyer: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelCopyFormatData {
    pub full: String,
    pub login_data: String,
    pub title_link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelCustomFields {
    #[serde(rename = "_4")]
    pub field_4: String,
    #[serde(rename = "allowSelfUnban")]
    pub allow_self_unban: Vec<serde_json::Value>,
    pub ban_reason: String,
    pub discord: String,
    pub github: String,
    pub jabber: String,
    #[serde(rename = "lztUnbanAmount")]
    pub lzt_unban_amount: String,
    pub steam: String,
    pub telegram: String,
    pub vk: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelExtraPricesItem {
    pub currency: String,
    pub price: String,
    #[serde(rename = "priceValue")]
    pub price_value: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelGuarantee {
    pub active: bool,
    pub cancelled: bool,
    #[serde(rename = "cancelledReason")]
    pub cancelled_reason: String,
    #[serde(rename = "cancelledReasonPhrase")]
    pub cancelled_reason_phrase: String,
    pub class: String,
    pub duration: i64,
    #[serde(rename = "durationPhrase")]
    pub duration_phrase: String,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "remainingTime")]
    pub remaining_time: i64,
    #[serde(rename = "remainingTimePhrase")]
    pub remaining_time_phrase: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelLoginData {
    #[serde(rename = "encodedOldPassword")]
    pub encoded_old_password: serde_json::Value,
    #[serde(rename = "encodedPassword")]
    pub encoded_password: String,
    #[serde(rename = "encodedRaw")]
    pub encoded_raw: String,
    pub login: String,
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    pub password: String,
    pub raw: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelSeller {
    pub active_items_count: i64,
    pub avatar_date: i64,
    pub contacts: MarketItemModelSellerContacts,
    pub display_style_group_id: i64,
    pub effective_last_activity: i64,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    pub is_banned: i64,
    pub joined_date: i64,
    pub restore_data: String,
    pub restore_percents: serde_json::Value,
    pub sold_items_count: i64,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelSellerContacts {
    pub ban_reason: String,
    pub telegram: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelTags {
    #[serde(rename = "1234567890")]
    pub field_1234567890: MarketItemModelTags1234567890,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketItemModelTags1234567890 {
    pub bc: String,
    #[serde(rename = "forOwnedAccountsOnly")]
    pub for_owned_accounts_only: bool,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    pub tag_id: i64,
    pub title: String,
}


pub type MarketRandomProxy = bool;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketRespSystemInfo {
    pub log_id: i64,
    pub time: i64,
    pub visitor_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModel {
    pub active_items_count: i64,
    pub activity_visible: bool,
    pub age: i64,
    pub balance: String,
    pub balances: Vec<MarketUserModelBalancesItem>,
    pub bump_item_period: i64,
    pub can_edit: bool,
    pub can_follow: bool,
    pub can_ignore: bool,
    pub can_post_profile: bool,
    pub can_view_profile: bool,
    pub can_view_profile_posts: bool,
    pub can_warn: bool,
    pub contest_count: i64,
    pub conv_welcome_message: String,
    #[serde(rename = "convertedBalance")]
    pub converted_balance: i64,
    #[serde(rename = "convertedDeposit")]
    pub converted_deposit: i64,
    #[serde(rename = "convertedHold")]
    pub converted_hold: i64,
    pub currency: String,
    #[serde(rename = "currencyPhrase")]
    pub currency_phrase: String,
    pub custom_account_download_format: String,
    pub custom_fields: MarketUserModelCustomFields,
    pub custom_title: String,
    pub deposit: i64,
    pub dob: MarketUserModelDob,
    pub feedback_data: MarketUserModelFeedbackData,
    pub hold: String,
    pub homepage: String,
    pub imap_data: MarketUserModelImapData,
    pub is_admin: bool,
    pub is_banned: bool,
    pub is_followed: bool,
    pub is_ignored: bool,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub is_super_admin: bool,
    pub joined_date: i64,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub location: String,
    pub market_custom_title: String,
    pub max_discount_percent: i64,
    pub message_count: i64,
    pub paid_mail_left: i64,
    pub public_tags: Vec<MarketUserModelPublicTagsItem>,
    pub register_date: i64,
    pub rendered: MarketUserModelRendered,
    pub restore_count: i64,
    pub restore_data: MarketUserModelRestoreData,
    pub short_link: String,
    pub sold_items_count: i64,
    pub tags: Vec<MarketUserModelTagsItem>,
    pub telegram_client: MarketUserModelTelegramClient,
    pub trophy_points: i64,
    pub user_allow_ask_discount: bool,
    pub user_id: i64,
    pub user_title: String,
    pub username: String,
    pub view_url: String,
    pub visible: bool,
    pub warning_points: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelBalancesItem {
    pub balance: String,
    pub balance_id: i64,
    #[serde(rename = "convertedBalance")]
    pub converted_balance: f64,
    pub custom_title: serde_json::Value,
    #[serde(rename = "fullTitle")]
    pub full_title: String,
    pub merchant_id: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelCustomFields {
    #[serde(rename = "_4")]
    pub field_4: String,
    #[serde(rename = "allowSelfUnban")]
    pub allow_self_unban: Vec<serde_json::Value>,
    pub ban_reason: String,
    pub discord: String,
    #[serde(rename = "favoriteAnime")]
    pub favorite_anime: String,
    #[serde(rename = "favoritePorn")]
    pub favorite_porn: String,
    #[serde(rename = "favoriteVape")]
    pub favorite_vape: String,
    pub github: String,
    pub jabber: String,
    #[serde(rename = "lztAwardUserTrophy")]
    pub lzt_award_user_trophy: String,
    #[serde(rename = "lztLikesIncreasing")]
    pub lzt_likes_increasing: String,
    #[serde(rename = "lztLikesZeroing")]
    pub lzt_likes_zeroing: String,
    #[serde(rename = "lztSympathyIncreasing")]
    pub lzt_sympathy_increasing: String,
    #[serde(rename = "lztSympathyZeroing")]
    pub lzt_sympathy_zeroing: String,
    #[serde(rename = "lztUnbanAmount")]
    pub lzt_unban_amount: String,
    #[serde(rename = "maecenasValue")]
    pub maecenas_value: String,
    pub matrix: String,
    #[serde(rename = "scamURL")]
    pub scam_url: String,
    pub steam: String,
    pub telegram: String,
    pub vk: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelDob {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelFeedbackData {
    #[serde(rename = "12345")]
    pub field_12345: MarketUserModelFeedbackData12345,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelFeedbackData12345 {
    pub negative: i64,
    pub positive: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelImapData {
    #[serde(rename = "domain.zone")]
    pub domain_zone: MarketUserModelImapDataDomainZone,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelImapDataDomainZone {
    pub domain: String,
    pub imap_server: String,
    pub port: i64,
    pub secure: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelPublicTagsItem {
    pub background_color: String,
    pub tag_id: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelRendered {
    pub avatars: MarketUserModelRenderedAvatars,
    pub backgrounds: MarketUserModelRenderedBackgrounds,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelRenderedBackgrounds {
    pub l: String,
    pub m: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelRestoreData {
    #[serde(rename = "12345")]
    pub field_12345: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelTagsItem {
    pub bc: String,
    #[serde(rename = "forOwnedAccountsOnly")]
    pub for_owned_accounts_only: bool,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    pub tag_id: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketUserModelTelegramClient {
    pub telegram_api_hash: String,
    pub telegram_api_id: String,
    pub telegram_app_version: String,
    pub telegram_device_model: String,
    pub telegram_lang_code: String,
    pub telegram_lang_pack: String,
    pub telegram_system_lang_code: String,
    pub telegram_system_version: String,
}


pub type MarketYesNoNoMatterScheme = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsBalanceExchangeRequest {
    pub amount: i64,
    pub from_balance: String,
    pub to_balance: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCancelRequest {
    pub payment_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponse {
    #[serde(rename = "currencyList")]
    pub currency_list: PaymentsCurrencyResponseCurrencyList,
    #[serde(rename = "lastUpdate")]
    pub last_update: i64,
    pub system_info: MarketRespSystemInfo,
    #[serde(rename = "visitorCurrency")]
    pub visitor_currency: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyList {
    #[serde(rename = "AED")]
    pub aed: PaymentsCurrencyResponseCurrencyListAed,
    #[serde(rename = "ARS")]
    pub ars: PaymentsCurrencyResponseCurrencyListArs,
    #[serde(rename = "AUD")]
    pub aud: PaymentsCurrencyResponseCurrencyListAud,
    #[serde(rename = "BCH")]
    pub bch: PaymentsCurrencyResponseCurrencyListBch,
    #[serde(rename = "BGN")]
    pub bgn: PaymentsCurrencyResponseCurrencyListBgn,
    #[serde(rename = "BNB")]
    pub bnb: PaymentsCurrencyResponseCurrencyListBnb,
    #[serde(rename = "BRL")]
    pub brl: PaymentsCurrencyResponseCurrencyListBrl,
    #[serde(rename = "BTC")]
    pub btc: PaymentsCurrencyResponseCurrencyListBtc,
    #[serde(rename = "CAD")]
    pub cad: PaymentsCurrencyResponseCurrencyListCad,
    #[serde(rename = "CHF")]
    pub chf: PaymentsCurrencyResponseCurrencyListChf,
    #[serde(rename = "CLP")]
    pub clp: PaymentsCurrencyResponseCurrencyListClp,
    #[serde(rename = "CNY")]
    pub cny: PaymentsCurrencyResponseCurrencyListCny,
    #[serde(rename = "COP")]
    pub cop: PaymentsCurrencyResponseCurrencyListCop,
    #[serde(rename = "CRC")]
    pub crc: PaymentsCurrencyResponseCurrencyListCrc,
    #[serde(rename = "CZK")]
    pub czk: PaymentsCurrencyResponseCurrencyListCzk,
    #[serde(rename = "DASH")]
    pub dash: PaymentsCurrencyResponseCurrencyListDash,
    #[serde(rename = "DKK")]
    pub dkk: PaymentsCurrencyResponseCurrencyListDkk,
    #[serde(rename = "DOGE")]
    pub doge: PaymentsCurrencyResponseCurrencyListDoge,
    #[serde(rename = "ETH")]
    pub eth: PaymentsCurrencyResponseCurrencyListEth,
    #[serde(rename = "EUR")]
    pub eur: PaymentsCurrencyResponseCurrencyListEur,
    #[serde(rename = "GBP")]
    pub gbp: PaymentsCurrencyResponseCurrencyListGbp,
    #[serde(rename = "GEL")]
    pub gel: PaymentsCurrencyResponseCurrencyListGel,
    #[serde(rename = "HKD")]
    pub hkd: PaymentsCurrencyResponseCurrencyListHkd,
    #[serde(rename = "HUF")]
    pub huf: PaymentsCurrencyResponseCurrencyListHuf,
    #[serde(rename = "IDR")]
    pub idr: PaymentsCurrencyResponseCurrencyListIdr,
    #[serde(rename = "ILS")]
    pub ils: PaymentsCurrencyResponseCurrencyListIls,
    #[serde(rename = "INR")]
    pub inr: PaymentsCurrencyResponseCurrencyListInr,
    #[serde(rename = "JPY")]
    pub jpy: PaymentsCurrencyResponseCurrencyListJpy,
    #[serde(rename = "KRW")]
    pub krw: PaymentsCurrencyResponseCurrencyListKrw,
    #[serde(rename = "KWD")]
    pub kwd: PaymentsCurrencyResponseCurrencyListKwd,
    #[serde(rename = "KZT")]
    pub kzt: PaymentsCurrencyResponseCurrencyListKzt,
    #[serde(rename = "LTC")]
    pub ltc: PaymentsCurrencyResponseCurrencyListLtc,
    #[serde(rename = "MATIC")]
    pub matic: PaymentsCurrencyResponseCurrencyListMatic,
    #[serde(rename = "MXN")]
    pub mxn: PaymentsCurrencyResponseCurrencyListMxn,
    #[serde(rename = "MYR")]
    pub myr: PaymentsCurrencyResponseCurrencyListMyr,
    #[serde(rename = "NOK")]
    pub nok: PaymentsCurrencyResponseCurrencyListNok,
    #[serde(rename = "NZD")]
    pub nzd: PaymentsCurrencyResponseCurrencyListNzd,
    #[serde(rename = "PEN")]
    pub pen: PaymentsCurrencyResponseCurrencyListPen,
    #[serde(rename = "PHP")]
    pub php: PaymentsCurrencyResponseCurrencyListPhp,
    #[serde(rename = "PLN")]
    pub pln: PaymentsCurrencyResponseCurrencyListPln,
    #[serde(rename = "QAR")]
    pub qar: PaymentsCurrencyResponseCurrencyListQar,
    #[serde(rename = "RON")]
    pub ron: PaymentsCurrencyResponseCurrencyListRon,
    #[serde(rename = "RSD")]
    pub rsd: PaymentsCurrencyResponseCurrencyListRsd,
    #[serde(rename = "RUB")]
    pub rub: PaymentsCurrencyResponseCurrencyListRub,
    #[serde(rename = "SAR")]
    pub sar: PaymentsCurrencyResponseCurrencyListSar,
    #[serde(rename = "SEK")]
    pub sek: PaymentsCurrencyResponseCurrencyListSek,
    #[serde(rename = "SGD")]
    pub sgd: PaymentsCurrencyResponseCurrencyListSgd,
    #[serde(rename = "SOL")]
    pub sol: PaymentsCurrencyResponseCurrencyListSol,
    #[serde(rename = "THB")]
    pub thb: PaymentsCurrencyResponseCurrencyListThb,
    #[serde(rename = "TON")]
    pub ton: PaymentsCurrencyResponseCurrencyListTon,
    #[serde(rename = "TRX")]
    pub trx: PaymentsCurrencyResponseCurrencyListTrx,
    #[serde(rename = "TRY")]
    pub try_: PaymentsCurrencyResponseCurrencyListTry,
    #[serde(rename = "TWD")]
    pub twd: PaymentsCurrencyResponseCurrencyListTwd,
    #[serde(rename = "UAH")]
    pub uah: PaymentsCurrencyResponseCurrencyListUah,
    #[serde(rename = "USD")]
    pub usd: PaymentsCurrencyResponseCurrencyListUsd,
    #[serde(rename = "USDT")]
    pub usdt: PaymentsCurrencyResponseCurrencyListUsdt,
    #[serde(rename = "UYU")]
    pub uyu: PaymentsCurrencyResponseCurrencyListUyu,
    #[serde(rename = "VND")]
    pub vnd: PaymentsCurrencyResponseCurrencyListVnd,
    #[serde(rename = "XMR")]
    pub xmr: PaymentsCurrencyResponseCurrencyListXmr,
    #[serde(rename = "ZAR")]
    pub zar: PaymentsCurrencyResponseCurrencyListZar,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListAed {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListArs {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListAud {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListBch {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListBgn {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListBnb {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListBrl {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListBtc {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListCad {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListChf {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListClp {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListCny {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListCop {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListCrc {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListCzk {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListDash {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListDkk {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListDoge {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListEth {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListEur {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListGbp {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListGel {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListHkd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListHuf {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListIdr {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListIls {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListInr {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListJpy {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListKrw {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListKwd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListKzt {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListLtc {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListMatic {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListMxn {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListMyr {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListNok {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListNzd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListPen {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListPhp {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListPln {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListQar {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListRon {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListRsd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListRub {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: i64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListSar {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListSek {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListSgd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListSol {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListThb {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListTon {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListTrx {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListTry {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListTwd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListUah {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListUsd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListUsdt {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListUyu {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListVnd {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListXmr {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponseCurrencyListZar {
    #[serde(rename = "formattedRate")]
    pub formatted_rate: String,
    pub rate: f64,
    pub symbol: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsFeeResponse {
    pub calculator: PaymentsFeeResponseCalculator,
    pub commission_percentage: i64,
    #[serde(rename = "spentCurrentMonth")]
    pub spent_current_month: i64,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsFeeResponseCalculator {
    #[serde(rename = "commissionAmount")]
    pub commission_amount: i64,
    #[serde(rename = "inputAmount")]
    pub input_amount: i64,
    #[serde(rename = "totalOutputAmount")]
    pub total_output_amount: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponse {
    #[serde(rename = "filterDatesDefault")]
    pub filter_dates_default: bool,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub input: PaymentsHistoryResponseInput,
    #[serde(rename = "lastOperationId")]
    pub last_operation_id: i64,
    #[serde(rename = "nextPageHref")]
    pub next_page_href: String,
    pub page: i64,
    #[serde(rename = "pageNavLink")]
    pub page_nav_link: String,
    #[serde(rename = "pageNavParams")]
    pub page_nav_params: PaymentsHistoryResponsePageNavParams,
    #[serde(rename = "paymentStats")]
    pub payment_stats: serde_json::Value,
    pub payments: PaymentsHistoryResponsePayments,
    #[serde(rename = "perPage")]
    pub per_page: String,
    #[serde(rename = "periodLabel")]
    pub period_label: String,
    #[serde(rename = "periodLabelPhrase")]
    pub period_label_phrase: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponseInput {
    pub category_id: i64,
    pub comment: String,
    pub currency: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub is_hold: bool,
    pub operation_id_lt: i64,
    pub page: i64,
    pub period_label: String,
    pub pmax: String,
    pub pmin: String,
    pub receiver: String,
    pub sender: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user_id: i64,
    pub wallet: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePageNavParams {
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePayments {
    #[serde(rename = "1234567890")]
    pub field_1234567890: PaymentsHistoryResponsePayments1234567890,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePayments1234567890 {
    pub api: i64,
    #[serde(rename = "canCancelBalanceHold")]
    pub can_cancel_balance_hold: bool,
    #[serde(rename = "canCancelBalancePayout")]
    pub can_cancel_balance_payout: bool,
    #[serde(rename = "canCancelBalanceTransfer")]
    pub can_cancel_balance_transfer: bool,
    #[serde(rename = "canCancelPaidMailPayment")]
    pub can_cancel_paid_mail_payment: bool,
    #[serde(rename = "canFinishBalanceHold")]
    pub can_finish_balance_hold: bool,
    #[serde(rename = "canFinishBalancePayout")]
    pub can_finish_balance_payout: bool,
    #[serde(rename = "canFinishBalanceTransfer")]
    pub can_finish_balance_transfer: bool,
    pub data: PaymentsHistoryResponsePayments1234567890Data,
    pub hold_end_date: i64,
    pub incoming_sum: String,
    pub is_finished: i64,
    pub is_hold: i64,
    pub item_id: i64,
    pub label: PaymentsHistoryResponsePayments1234567890Label,
    pub merchant: PaymentsHistoryResponsePayments1234567890Merchant,
    pub operation_date: i64,
    pub operation_end_date: i64,
    pub operation_id: i64,
    pub operation_type: String,
    pub outgoing_sum: String,
    #[serde(rename = "paymentSystemIcons")]
    pub payment_system_icons: Vec<serde_json::Value>,
    pub payment_status: String,
    pub payment_system: String,
    pub sum: String,
    #[serde(rename = "supportLink")]
    pub support_link: serde_json::Value,
    pub user: PaymentsHistoryResponsePayments1234567890User,
    pub wallet: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePayments1234567890Data {
    pub avatar: String,
    pub avatar_date: i64,
    pub comment: String,
    #[serde(rename = "commentPlain")]
    pub comment_plain: String,
    pub display_style_group_id: i64,
    pub fee: i64,
    pub invoice_id: i64,
    pub is_banned: i64,
    pub is_test: bool,
    pub payment_id: String,
    pub uniq_banner: String,
    pub uniq_username_css: String,
    pub user_group_id: i64,
    pub user_id: i64,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePayments1234567890Label {
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePayments1234567890Merchant {
    pub avatar_data: String,
    pub created_date: i64,
    pub merchant_id: i64,
    pub name: String,
    pub secret_key: String,
    pub url: String,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponsePayments1234567890User {
    pub user_balance: String,
    pub user_balance_with_hold: f64,
    pub user_hold: String,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    pub amount: f64,
    pub comment: String,
    pub currency: MarketCurrencyModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<f64>,
    pub merchant_id: i64,
    pub payment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_telegram_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_callback: Option<String>,
    pub url_success: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceCreateResponse {
    pub invoice: MarketInvoiceModel,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceGetResponse {
    pub invoice: MarketInvoiceModel,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceListResponse {
    pub count: i64,
    pub invoices: Vec<MarketInvoiceModel>,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutRequest {
    pub amount: f64,
    pub currency: MarketCurrencyModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<PaymentsPayoutRequestExtra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fee: Option<bool>,
    pub payment_system: String,
    pub wallet: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutRequestExtra {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponse {
    pub system_info: MarketRespSystemInfo,
    pub systems: Vec<PaymentsPayoutServicesResponseSystemsItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItem {
    pub commission: String,
    pub has_wallet: bool,
    pub instant_payout: bool,
    pub is_unavailable: bool,
    pub max: i64,
    pub min: i64,
    pub p2p: bool,
    pub problematic_payout: bool,
    pub providers: PaymentsPayoutServicesResponseSystemsItemProviders,
    pub system: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProviders {
    #[serde(rename = "BCH")]
    pub bch: PaymentsPayoutServicesResponseSystemsItemProvidersBch,
    #[serde(rename = "BEP20")]
    pub bep20: PaymentsPayoutServicesResponseSystemsItemProvidersBep20,
    #[serde(rename = "BNB")]
    pub bnb: PaymentsPayoutServicesResponseSystemsItemProvidersBnb,
    #[serde(rename = "BTC")]
    pub btc: PaymentsPayoutServicesResponseSystemsItemProvidersBtc,
    #[serde(rename = "DASH")]
    pub dash: PaymentsPayoutServicesResponseSystemsItemProvidersDash,
    #[serde(rename = "DOGE")]
    pub doge: PaymentsPayoutServicesResponseSystemsItemProvidersDoge,
    #[serde(rename = "ERC20")]
    pub erc20: PaymentsPayoutServicesResponseSystemsItemProvidersErc20,
    #[serde(rename = "ETH")]
    pub eth: PaymentsPayoutServicesResponseSystemsItemProvidersEth,
    #[serde(rename = "LTC")]
    pub ltc: PaymentsPayoutServicesResponseSystemsItemProvidersLtc,
    #[serde(rename = "SOL")]
    pub sol: PaymentsPayoutServicesResponseSystemsItemProvidersSol,
    #[serde(rename = "TON")]
    pub ton: PaymentsPayoutServicesResponseSystemsItemProvidersTon,
    #[serde(rename = "TRC20")]
    pub trc20: PaymentsPayoutServicesResponseSystemsItemProvidersTrc20,
    #[serde(rename = "TRX")]
    pub trx: PaymentsPayoutServicesResponseSystemsItemProvidersTrx,
    #[serde(rename = "XMR")]
    pub xmr: PaymentsPayoutServicesResponseSystemsItemProvidersXmr,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersBch {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersBep20 {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersBnb {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersBtc {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersDash {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersDoge {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersErc20 {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersEth {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersLtc {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersSol {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersTon {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersTrc20 {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersTrx {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponseSystemsItemProvidersXmr {
    #[serde(rename = "isUnavailable")]
    pub is_unavailable: bool,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsTransferRequest {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub currency: MarketCurrencyModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_length_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_length_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_deal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_hold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponse {
    pub claims: Vec<ProfileClaimsResponseClaimsItem>,
    pub stats: ProfileClaimsResponseStats,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseClaimsItem {
    pub amount_formatted: String,
    pub author: ProfileClaimsResponseClaimsItemAuthor,
    pub claim_date: i64,
    pub claim_state: String,
    pub message_body: String,
    pub thread_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseClaimsItemAuthor {
    pub ban_reason: String,
    pub contest_count: i64,
    pub custom_title: String,
    pub fields: Vec<ProfileClaimsResponseClaimsItemAuthorFieldsItem>,
    pub is_banned: i64,
    pub links: ProfileClaimsResponseClaimsItemAuthorLinks,
    pub permissions: ProfileClaimsResponseClaimsItemAuthorPermissions,
    pub trophy_count: i64,
    pub user_group_id: i64,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_title: String,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseClaimsItemAuthorFieldsItem {
    pub description: String,
    pub id: String,
    pub is_required: bool,
    pub position: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseClaimsItemAuthorLinks {
    pub avatar: String,
    pub avatar_big: String,
    pub avatar_small: String,
    pub detail: String,
    pub followers: String,
    pub followings: String,
    pub ignore: String,
    pub permalink: String,
    pub timeline: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseClaimsItemAuthorPermissions {
    pub edit: bool,
    pub follow: bool,
    pub ignore: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseStats {
    pub market: ProfileClaimsResponseStatsMarket,
    #[serde(rename = "noMarket")]
    pub no_market: ProfileClaimsResponseStatsNoMarket,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseStatsMarket {
    pub rejected: i64,
    pub settled: i64,
    pub solved: i64,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponseStatsNoMarket {
    pub rejected: i64,
    pub settled: i64,
    pub solved: i64,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_accept_accounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_telegram_client: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<ProfileEditRequestOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_api_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_api_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_device_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_lang_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_lang_pack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_system_lang_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_system_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ProfileEditRequestUser>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileEditRequestOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_password_on_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deauthorize_steam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_steam_guard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_favourites: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_too_low_price_change_warning: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileEditRequestUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<MarketCurrencyModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_custom_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount_percent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_allow_ask_discount: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileGetResponse {
    pub system_info: MarketRespSystemInfo,
    pub user: MarketUserModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyAddRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_pass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_row: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyDeleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGetResponse {
    pub proxies: Vec<ProxyGetResponseProxiesItem>,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGetResponseProxiesItem {
    pub proxy: ProxyGetResponseProxiesItemProxy,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGetResponseProxiesItemProxy {
    #[serde(rename = "proxyString")]
    pub proxy_string: String,
    pub proxy_id: i64,
    pub proxy_ip: String,
    pub proxy_pass: String,
    pub proxy_port: i64,
    pub proxy_user: String,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingAddRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<bool>,
    pub category_id: i64,
    pub currency: MarketCurrencyModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_guarantee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forceTempEmail")]
    pub force_temp_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    pub item_origin: String,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_proxy: Option<MarketRandomProxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resell_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingAddResponse {
    pub item: MarketItemModel,
    pub status: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingCheckRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<MarketExtraModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resell_item_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingExternalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingFastSellRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<bool>,
    pub category_id: i64,
    pub currency: MarketCurrencyModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_guarantee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<MarketExtraModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    pub item_origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_proxy: Option<MarketRandomProxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingFastSellResponse {
    pub item: MarketItemModel,
    #[serde(rename = "itemLink")]
    pub item_link: String,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingConfirmRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingConfirmResponse {
    pub item: PurchasingConfirmResponseItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub system_info: MarketRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingConfirmResponseItem {
    #[serde(rename = "loginData")]
    pub login_data: PurchasingConfirmResponseItemLoginData,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingConfirmResponseItemLoginData {
    #[serde(rename = "adviceToChangePassword")]
    pub advice_to_change_password: bool,
    #[serde(rename = "encodedOldPassword")]
    pub encoded_old_password: String,
    #[serde(rename = "encodedPassword")]
    pub encoded_password: String,
    #[serde(rename = "encodedRaw")]
    pub encoded_raw: String,
    pub login: String,
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    pub password: String,
    pub raw: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingDiscountRequestRequest {
    pub discount_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingFastBuyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}


