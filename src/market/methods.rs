//! Auto-generated methods for Lolzteam Public API: Market.
//! DO NOT EDIT -- run `cargo run --bin lzt-codegen -- generate`.

#![allow(clippy::all)]

use crate::client::HttpMethod;
use crate::error::Result;
use crate::market::types::*;

impl crate::market::MarketApi {
    // -- Category Search --
    /// `GET /`
    pub async fn category_all(&self, query: CategoryAllQuery) -> Result<serde_json::Value> {
        let path = "/".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Payments --
    /// `POST /auto-payment`
    pub async fn auto_payments_create(&self, body: AutoPaymentsCreateRequest) -> Result<AutoPaymentsCreateResponse> {
        let path = "/auto-payment".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /auto-payment`
    pub async fn auto_payments_delete(&self, body: AutoPaymentsDeleteRequest) -> Result<serde_json::Value> {
        let path = "/auto-payment".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /auto-payments`
    pub async fn auto_payments_list(&self) -> Result<AutoPaymentsListResponse> {
        let path = "/auto-payments".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /balance/exchange`
    pub async fn payments_balance_list(&self) -> Result<serde_json::Value> {
        let path = "/balance/exchange".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /balance/exchange`
    pub async fn payments_balance_exchange(&self, body: PaymentsBalanceExchangeRequest) -> Result<serde_json::Value> {
        let path = "/balance/exchange".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /balance/payout`
    pub async fn payments_payout(&self, body: PaymentsPayoutRequest) -> Result<serde_json::Value> {
        let path = "/balance/payout".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /balance/payout/services`
    pub async fn payments_payout_services(&self) -> Result<PaymentsPayoutServicesResponse> {
        let path = "/balance/payout/services".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /balance/transfer`
    pub async fn payments_transfer(&self, body: PaymentsTransferRequest) -> Result<serde_json::Value> {
        let path = "/balance/transfer".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /balance/transfer/cancel`
    pub async fn payments_cancel(&self, body: PaymentsCancelRequest) -> Result<serde_json::Value> {
        let path = "/balance/transfer/cancel".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /balance/transfer/fee`
    pub async fn payments_fee(&self, query: PaymentsFeeQuery) -> Result<PaymentsFeeResponse> {
        let path = "/balance/transfer/fee".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Batch requests --
    /// `POST /batch`
    pub async fn batch(&self, body: BatchRequest) -> Result<BatchResponse> {
        let path = "/batch".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Category Search --
    /// `GET /battlenet`
    pub async fn category_battle_net(&self, query: CategoryBattleNetQuery) -> Result<CategoryBattleNetResponse> {
        let path = "/battlenet".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Accounts managing --
    /// `POST /bulk/items`
    pub async fn managing_bulk_get(&self, body: ManagingBulkGetRequest) -> Result<ManagingBulkGetResponse> {
        let path = "/bulk/items".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Cart --
    /// `GET /cart`
    pub async fn cart_get(&self, query: CartGetQuery) -> Result<serde_json::Value> {
        let path = "/cart".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /cart`
    pub async fn cart_add(&self, body: CartAddRequest) -> Result<CartAddResponse> {
        let path = "/cart".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /cart`
    pub async fn cart_delete(&self, body: CartDeleteRequest) -> Result<CartDeleteResponse> {
        let path = "/cart".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    // -- Categories --
    /// `GET /category`
    pub async fn category_list(&self, query: CategoryListQuery) -> Result<CategoryListResponse> {
        let path = "/category".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Category Search --
    /// `GET /chatgpt`
    pub async fn category_chat_gpt(&self, query: CategoryChatGptQuery) -> Result<CategoryChatGptResponse> {
        let path = "/chatgpt".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Accounts managing --
    /// `GET /claims`
    pub async fn profile_claims(&self, query: ProfileClaimsQuery) -> Result<ProfileClaimsResponse> {
        let path = "/claims".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /claims`
    pub async fn managing_create_claim(&self, body: ManagingCreateClaimRequest) -> Result<ManagingCreateClaimResponse> {
        let path = "/claims".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Payments --
    /// `GET /currency`
    pub async fn payments_currency(&self) -> Result<PaymentsCurrencyResponse> {
        let path = "/currency".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Custom Discounts --
    /// `GET /custom-discounts`
    pub async fn custom_discounts_get(&self) -> Result<CustomDiscountsGetResponse> {
        let path = "/custom-discounts".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /custom-discounts`
    pub async fn custom_discounts_create(&self, body: CustomDiscountsCreateRequest) -> Result<CustomDiscountsCreateResponse> {
        let path = "/custom-discounts".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `PUT /custom-discounts`
    pub async fn custom_discounts_edit(&self, body: CustomDiscountsEditRequest) -> Result<CustomDiscountsEditResponse> {
        let path = "/custom-discounts".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /custom-discounts`
    pub async fn custom_discounts_delete(&self, body: CustomDiscountsDeleteRequest) -> Result<serde_json::Value> {
        let path = "/custom-discounts".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    // -- Category Search --
    /// `GET /discord`
    pub async fn category_discord(&self, query: CategoryDiscordQuery) -> Result<CategoryDiscordResponse> {
        let path = "/discord".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /ea`
    pub async fn category_ea(&self, query: CategoryEaQuery) -> Result<CategoryEaResponse> {
        let path = "/ea".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /epicgames`
    pub async fn category_epic_games(&self, query: CategoryEpicGamesQuery) -> Result<CategoryEpicGamesResponse> {
        let path = "/epicgames".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /escape-from-tarkov`
    pub async fn category_escape_from_tarkov(&self, query: CategoryEscapeFromTarkovQuery) -> Result<CategoryEscapeFromTarkovResponse> {
        let path = "/escape-from-tarkov".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Accounts list --
    /// `GET /fave`
    pub async fn list_favorites(&self, query: ListFavoritesQuery) -> Result<ListFavoritesResponse> {
        let path = "/fave".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Category Search --
    /// `GET /fortnite`
    pub async fn category_fortnite(&self, query: CategoryFortniteQuery) -> Result<CategoryFortniteResponse> {
        let path = "/fortnite".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /gifts`
    pub async fn category_gifts(&self, query: CategoryGiftsQuery) -> Result<CategoryGiftsResponse> {
        let path = "/gifts".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /hytale`
    pub async fn category_hytale(&self, query: CategoryHytaleQuery) -> Result<CategoryHytaleResponse> {
        let path = "/hytale".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- IMAP --
    /// `POST /imap`
    pub async fn imap_create(&self, body: ImapCreateRequest) -> Result<serde_json::Value> {
        let path = "/imap".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /imap`
    pub async fn imap_delete(&self, body: ImapDeleteRequest) -> Result<serde_json::Value> {
        let path = "/imap".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    // -- Category Search --
    /// `GET /instagram`
    pub async fn category_instagram(&self, query: CategoryInstagramQuery) -> Result<CategoryInstagramResponse> {
        let path = "/instagram".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Invoices --
    /// `GET /invoice`
    pub async fn payments_invoice_get(&self, query: PaymentsInvoiceGetQuery) -> Result<PaymentsInvoiceGetResponse> {
        let path = "/invoice".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /invoice`
    pub async fn payments_invoice_create(&self, body: PaymentsInvoiceCreateRequest) -> Result<PaymentsInvoiceCreateResponse> {
        let path = "/invoice".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /invoice/list`
    pub async fn payments_invoice_list(&self, query: PaymentsInvoiceListQuery) -> Result<PaymentsInvoiceListResponse> {
        let path = "/invoice/list".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Account publishing --
    /// `POST /item/add`
    pub async fn publishing_add(&self, body: PublishingAddRequest) -> Result<PublishingAddResponse> {
        let path = "/item/add".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /item/fast-sell`
    pub async fn publishing_fast_sell(&self, body: PublishingFastSellRequest) -> Result<PublishingFastSellResponse> {
        let path = "/item/fast-sell".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Accounts managing --
    /// `GET /letters2`
    pub async fn managing_get_letters2(&self, query: ManagingGetLetters2Query) -> Result<ManagingGetLetters2Response> {
        let path = "/letters2".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Profile --
    /// `GET /me`
    pub async fn profile_get(&self, query: ProfileGetQuery) -> Result<ProfileGetResponse> {
        let path = "/me".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `PUT /me`
    pub async fn profile_edit(&self, body: ProfileEditRequest) -> Result<serde_json::Value> {
        let path = "/me".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    // -- Category Search --
    /// `GET /mihoyo`
    pub async fn category_mihoyo(&self, query: CategoryMihoyoQuery) -> Result<CategoryMihoyoResponse> {
        let path = "/mihoyo".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /minecraft`
    pub async fn category_minecraft(&self, query: CategoryMinecraftQuery) -> Result<CategoryMinecraftResponse> {
        let path = "/minecraft".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Proxy --
    /// `GET /proxy`
    pub async fn proxy_get(&self) -> Result<ProxyGetResponse> {
        let path = "/proxy".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /proxy`
    pub async fn proxy_add(&self, body: ProxyAddRequest) -> Result<serde_json::Value> {
        let path = "/proxy".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /proxy`
    pub async fn proxy_delete(&self, body: ProxyDeleteRequest) -> Result<serde_json::Value> {
        let path = "/proxy".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    // -- Category Search --
    /// `GET /riot`
    pub async fn category_riot(&self, query: CategoryRiotQuery) -> Result<CategoryRiotResponse> {
        let path = "/riot".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /roblox`
    pub async fn category_roblox(&self, query: CategoryRobloxQuery) -> Result<CategoryRobloxResponse> {
        let path = "/roblox".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /socialclub`
    pub async fn category_social_club(&self, query: CategorySocialClubQuery) -> Result<CategorySocialClubResponse> {
        let path = "/socialclub".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /steam`
    pub async fn category_steam(&self, query: CategorySteamQuery) -> Result<CategorySteamResponse> {
        let path = "/steam".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Accounts managing --
    /// `GET /steam-value`
    pub async fn managing_steam_value(&self, query: ManagingSteamValueQuery) -> Result<ManagingSteamValueResponse> {
        let path = "/steam-value".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Category Search --
    /// `GET /supercell`
    pub async fn category_supercell(&self, query: CategorySupercellQuery) -> Result<CategorySupercellResponse> {
        let path = "/supercell".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /telegram`
    pub async fn category_telegram(&self, query: CategoryTelegramQuery) -> Result<CategoryTelegramResponse> {
        let path = "/telegram".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /tiktok`
    pub async fn category_tik_tok(&self, query: CategoryTikTokQuery) -> Result<CategoryTikTokResponse> {
        let path = "/tiktok".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /uplay`
    pub async fn category_uplay(&self, query: CategoryUplayQuery) -> Result<CategoryUplayResponse> {
        let path = "/uplay".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Accounts list --
    /// `GET /user/item-states`
    pub async fn list_states(&self, query: ListStatesQuery) -> Result<ListStatesResponse> {
        let path = "/user/item-states".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /user/items`
    pub async fn list_user(&self, query: ListUserQuery) -> Result<ListUserResponse> {
        let path = "/user/items".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /user/orders`
    pub async fn list_orders(&self, query: ListOrdersQuery) -> Result<ListOrdersResponse> {
        let path = "/user/orders".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Payments --
    /// `GET /user/payments`
    pub async fn payments_history(&self, query: PaymentsHistoryQuery) -> Result<PaymentsHistoryResponse> {
        let path = "/user/payments".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Accounts list --
    /// `GET /user/{type}/download`
    pub async fn list_download(&self, type_: String, query: ListDownloadQuery) -> Result<serde_json::Value> {
        let path = format!("/user/{type}/download", type = type_);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /viewed`
    pub async fn list_viewed(&self, query: ListViewedQuery) -> Result<ListViewedResponse> {
        let path = "/viewed".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Category Search --
    /// `GET /vpn`
    pub async fn category_vpn(&self, query: CategoryVpnQuery) -> Result<CategoryVpnResponse> {
        let path = "/vpn".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /warface`
    pub async fn category_warface(&self, query: CategoryWarfaceQuery) -> Result<CategoryWarfaceResponse> {
        let path = "/warface".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /world-of-tanks`
    pub async fn category_wot(&self, query: CategoryWotQuery) -> Result<CategoryWotResponse> {
        let path = "/world-of-tanks".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /wot-blitz`
    pub async fn category_wot_blitz(&self, query: CategoryWotBlitzQuery) -> Result<CategoryWotBlitzResponse> {
        let path = "/wot-blitz".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Categories --
    /// `GET /{categoryName}/games`
    pub async fn category_games(&self, category_name: String) -> Result<CategoryGamesResponse> {
        let path = format!("/{categoryName}/games", categoryName = category_name);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /{categoryName}/params`
    pub async fn category_params(&self, category_name: String) -> Result<CategoryParamsResponse> {
        let path = format!("/{categoryName}/params", categoryName = category_name);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Accounts managing --
    /// `GET /{item_id}`
    pub async fn managing_get(&self, item_id: i64, query: ManagingGetQuery) -> Result<ManagingGetResponse> {
        let path = format!("/{item_id}", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `DELETE /{item_id}`
    pub async fn manging_delete(&self, item_id: i64, body: MangingDeleteRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /{item_id}/ai-price`
    pub async fn managing_aiprice(&self, item_id: i64) -> Result<ManagingAipriceResponse> {
        let path = format!("/{item_id}/ai-price", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/auto-bump`
    pub async fn managing_auto_bump(&self, item_id: i64, body: ManagingAutoBumpRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/auto-bump", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /{item_id}/auto-bump`
    pub async fn managing_auto_bump_disable(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/auto-bump", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /{item_id}/auto-buy-price`
    pub async fn managing_auto_buy_price(&self, item_id: i64) -> Result<ManagingAutoBuyPriceResponse> {
        let path = format!("/{item_id}/auto-buy-price", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/bump`
    pub async fn managing_bump(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/bump", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/change-owner`
    pub async fn managing_transfer(&self, item_id: i64, body: ManagingTransferRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/change-owner", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /{item_id}/change-password`
    pub async fn managing_change_password(&self, item_id: i64, body: ManagingChangePasswordRequest) -> Result<ManagingChangePasswordResponse> {
        let path = format!("/{item_id}/change-password", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Account purchasing --
    /// `POST /{item_id}/check-account`
    pub async fn purchasing_check(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/check-account", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    // -- Accounts managing --
    /// `POST /{item_id}/check-guarantee`
    pub async fn managing_check_guarantee(&self, item_id: i64) -> Result<ManagingCheckGuaranteeResponse> {
        let path = format!("/{item_id}/check-guarantee", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/close`
    pub async fn managing_close(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/close", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    // -- Account purchasing --
    /// `POST /{item_id}/confirm-buy`
    pub async fn purchasing_confirm(&self, item_id: i64, body: PurchasingConfirmRequest) -> Result<PurchasingConfirmResponse> {
        let path = format!("/{item_id}/confirm-buy", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Accounts managing --
    /// `POST /{item_id}/confirm-sda`
    pub async fn managing_steam_sda(&self, item_id: i64, body: ManagingSteamSdaRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/confirm-sda", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /{item_id}/decline-video-recording`
    pub async fn managing_decline_video_recording(&self, item_id: i64, body: ManagingDeclineVideoRecordingRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/decline-video-recording", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Account purchasing --
    /// `POST /{item_id}/discount`
    pub async fn purchasing_discount_request(&self, item_id: i64, body: PurchasingDiscountRequestRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/discount", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /{item_id}/discount`
    pub async fn purchasing_discount_cancel(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/discount", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    // -- Accounts managing --
    /// `PUT /{item_id}/edit`
    pub async fn managing_edit(&self, item_id: i64, body: ManagingEditRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/edit", item_id = item_id);
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /{item_id}/email-code`
    pub async fn managing_email_code(&self, item_id: i64) -> Result<ManagingEmailCodeResponse> {
        let path = format!("/{item_id}/email-code", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Account publishing --
    /// `POST /{item_id}/external-account`
    pub async fn publishing_external(&self, item_id: i64, body: PublishingExternalRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/external-account", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Account purchasing --
    /// `POST /{item_id}/fast-buy`
    pub async fn purchasing_fast_buy(&self, item_id: i64, body: PurchasingFastBuyRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/fast-buy", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Account publishing --
    /// `POST /{item_id}/goods/check`
    pub async fn publishing_check(&self, item_id: i64, body: PublishingCheckRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/goods/check", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Accounts managing --
    /// `GET /{item_id}/guard-code`
    pub async fn managing_steam_mafile_code(&self, item_id: i64) -> Result<ManagingSteamMafileCodeResponse> {
        let path = format!("/{item_id}/guard-code", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /{item_id}/image`
    pub async fn managing_image(&self, item_id: i64, query: ManagingImageQuery) -> Result<ManagingImageResponse> {
        let path = format!("/{item_id}/image", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /{item_id}/inventory-value`
    pub async fn managing_steam_inventory_value(&self, item_id: i64, query: ManagingSteamInventoryValueQuery) -> Result<ManagingSteamInventoryValueResponse> {
        let path = format!("/{item_id}/inventory-value", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /{item_id}/mafile`
    pub async fn managing_steam_get_mafile(&self, item_id: i64) -> Result<ManagingSteamGetMafileResponse> {
        let path = format!("/{item_id}/mafile", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/mafile`
    pub async fn managing_steam_add_mafile(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/mafile", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /{item_id}/mafile`
    pub async fn managing_steam_remove_mafile(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/mafile", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/note-save`
    pub async fn managing_note(&self, item_id: i64, body: ManagingNoteRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/note-save", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /{item_id}/open`
    pub async fn managing_open(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/open", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/public-tag`
    pub async fn managing_public_tag(&self, item_id: i64, body: ManagingPublicTagRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/public-tag", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /{item_id}/public-tag`
    pub async fn managing_public_untag(&self, item_id: i64, body: ManagingPublicUntagRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/public-tag", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /{item_id}/refuse-guarantee`
    pub async fn managing_refuse_guarantee(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/refuse-guarantee", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/star`
    pub async fn managing_favorite(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/star", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /{item_id}/star`
    pub async fn managing_unfavorite(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/star", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /{item_id}/steam-preview`
    pub async fn managing_steam_preview(&self, item_id: i64, query: ManagingSteamPreviewQuery) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/steam-preview", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /{item_id}/stick`
    pub async fn managing_stick(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/stick", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /{item_id}/stick`
    pub async fn managing_unstick(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/stick", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/tag`
    pub async fn managing_tag(&self, item_id: i64, body: ManagingTagRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/tag", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /{item_id}/tag`
    pub async fn managing_untag(&self, item_id: i64, body: ManagingUntagRequest) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/tag", item_id = item_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /{item_id}/telegram-login-code`
    pub async fn managing_telegram_code(&self, item_id: i64) -> Result<ManagingTelegramCodeResponse> {
        let path = format!("/{item_id}/telegram-login-code", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/telegram-reset-authorizations`
    pub async fn managing_telegram_reset_auth(&self, item_id: i64) -> Result<serde_json::Value> {
        let path = format!("/{item_id}/telegram-reset-authorizations", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /{item_id}/temp-email-password`
    pub async fn managing_temp_email_password(&self, item_id: i64) -> Result<ManagingTempEmailPasswordResponse> {
        let path = format!("/{item_id}/temp-email-password", item_id = item_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /{item_id}/update-inventory`
    pub async fn managing_steam_update_value(&self, item_id: i64, body: ManagingSteamUpdateValueRequest) -> Result<ManagingSteamUpdateValueResponse> {
        let path = format!("/{item_id}/update-inventory", item_id = item_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

}
