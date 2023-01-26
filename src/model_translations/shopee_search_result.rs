use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopeeSearchResult {
    #[serde(rename = "bff_meta")]
    pub bff_meta: Value,
    pub error: Value,
    #[serde(rename = "error_msg")]
    pub error_msg: Value,
    #[serde(rename = "reserved_keyword")]
    pub reserved_keyword: Value,
    #[serde(rename = "suggestion_algorithm")]
    pub suggestion_algorithm: Value,
    pub algorithm: String,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub nomore: bool,
    pub items: Vec<Item>,
    // pub items: Value,
    #[serde(rename = "price_adjust")]
    pub price_adjust: Value,
    pub adjust: Adjust,
    #[serde(rename = "total_ads_count")]
    pub total_ads_count: i64,
    #[serde(rename = "hint_keywords")]
    pub hint_keywords: Value,
    #[serde(rename = "show_disclaimer")]
    pub show_disclaimer: bool,
    #[serde(rename = "json_data")]
    pub json_data: String,
    #[serde(rename = "query_rewrite")]
    pub query_rewrite: QueryRewrite,
    #[serde(rename = "disclaimer_infos")]
    // pub disclaimer_infos: Vec<Value>,
    pub disclaimer_infos: Value,
    #[serde(rename = "need_next_search")]
    pub need_next_search: bool,
    #[serde(rename = "low_result")]
    pub low_result: LowResult,
    #[serde(rename = "autoplay_info")]
    pub autoplay_info: Value,
    #[serde(rename = "food_item_info")]
    pub food_item_info: FoodItemInfo,
    #[serde(rename = "search_tracking")]
    pub search_tracking: String,
    #[serde(rename = "search_sessionid")]
    pub search_sessionid: Value,
    #[serde(rename = "batch_size")]
    pub batch_size: i64,
    #[serde(rename = "search_item_bff_tracking")]
    pub search_item_bff_tracking: String,
    #[serde(rename = "user_info")]
    pub user_info: UserInfo,
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "cached_result")]
    pub cached_result: Value,
    // pub experiments: Vec<Experiment>,
    pub experiments: Value,
    #[serde(rename = "item_extra_config")]
    pub item_extra_config: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "item_basic")]
    pub item_basic: ItemBasic,
    pub adsid: Option<i64>,
    pub campaignid: Option<i64>,
    pub distance: Value,
    #[serde(rename = "match_type")]
    pub match_type: Option<i64>,
    #[serde(rename = "ads_keyword")]
    pub ads_keyword: Option<String>,
    #[serde(rename = "deduction_info")]
    pub deduction_info: Option<String>,
    #[serde(rename = "collection_id")]
    pub collection_id: Value,
    #[serde(rename = "display_name")]
    pub display_name: Value,
    #[serde(rename = "campaign_stock")]
    pub campaign_stock: Value,
    #[serde(rename = "json_data")]
    pub json_data: String,
    #[serde(rename = "tracking_info")]
    pub tracking_info: Option<TrackingInfo>,
    pub itemid: i64,
    pub shopid: i64,
    #[serde(rename = "algo_image")]
    pub algo_image: Value,
    #[serde(rename = "fe_flags")]
    #[serde(default)]
    // pub fe_flags: Vec<i64>,
    pub fe_flags: Value,
    #[serde(rename = "item_type")]
    pub item_type: i64,
    #[serde(rename = "foody_item")]
    pub foody_item: Value,
    #[serde(rename = "search_item_tracking")]
    pub search_item_tracking: String,
    #[serde(rename = "bff_item_tracking")]
    pub bff_item_tracking: String,
    #[serde(rename = "personalized_labels")]
    pub personalized_labels: Value,
    #[serde(rename = "biz_json")]
    pub biz_json: Value,
    #[serde(rename = "creative_image_id")]
    pub creative_image_id: Option<String>,
    #[serde(rename = "creative_id")]
    pub creative_id: Option<String>,
    #[serde(rename = "creative_id_int")]
    pub creative_id_int: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemBasic {
    pub itemid: i64,
    pub shopid: i64,
    pub name: String,
    #[serde(rename = "label_ids")]
    // pub label_ids: Vec<i64>,
    pub label_ids: Value,
    pub image: String,
    pub images: Vec<String>,
    pub currency: String,
    pub stock: i64,
    pub status: i64,
    pub ctime: i64,
    pub sold: i64,
    #[serde(rename = "historical_sold")]
    pub historical_sold: i64,
    pub liked: bool,
    #[serde(rename = "liked_count")]
    pub liked_count: i64,
    #[serde(rename = "view_count")]
    pub view_count: Value,
    pub catid: i64,
    pub brand: String,
    #[serde(rename = "cmt_count")]
    pub cmt_count: i64,
    pub flag: i64,
    #[serde(rename = "cb_option")]
    pub cb_option: i64,
    #[serde(rename = "item_status")]
    pub item_status: String,
    pub price: i64,
    #[serde(rename = "price_min")]
    pub price_min: i64,
    #[serde(rename = "price_max")]
    pub price_max: i64,
    #[serde(rename = "price_min_before_discount")]
    pub price_min_before_discount: i64,
    #[serde(rename = "price_max_before_discount")]
    pub price_max_before_discount: i64,
    #[serde(rename = "hidden_price_display")]
    pub hidden_price_display: Value,
    #[serde(rename = "price_before_discount")]
    pub price_before_discount: i64,
    #[serde(rename = "has_lowest_price_guarantee")]
    pub has_lowest_price_guarantee: bool,
    #[serde(rename = "show_discount")]
    pub show_discount: i64,
    #[serde(rename = "raw_discount")]
    pub raw_discount: i64,
    pub discount: Option<String>,
    #[serde(rename = "is_category_failed")]
    pub is_category_failed: Value,
    #[serde(rename = "size_chart")]
    pub size_chart: Value,
    #[serde(rename = "video_info_list")]
    #[serde(default)]
    // pub video_info_list: Value,
    pub video_info_list: Option<Vec<VideoInfoList>>,
    #[serde(rename = "tier_variations")]
    // pub tier_variations: Value,
    pub tier_variations: Option<Vec<TierVariation>>,
    #[serde(rename = "item_rating")]
    pub item_rating: ItemRating,
    #[serde(rename = "item_type")]
    pub item_type: i64,
    #[serde(rename = "reference_item_id")]
    pub reference_item_id: String,
    #[serde(rename = "transparent_background_image")]
    pub transparent_background_image: String,
    #[serde(rename = "is_adult")]
    pub is_adult: bool,
    #[serde(rename = "badge_icon_type")]
    pub badge_icon_type: i64,
    #[serde(rename = "shopee_verified")]
    pub shopee_verified: bool,
    #[serde(rename = "is_official_shop")]
    pub is_official_shop: bool,
    #[serde(rename = "show_official_shop_label")]
    pub show_official_shop_label: bool,
    #[serde(rename = "show_shopee_verified_label")]
    pub show_shopee_verified_label: bool,
    #[serde(rename = "show_official_shop_label_in_title")]
    pub show_official_shop_label_in_title: bool,
    #[serde(rename = "is_cc_installment_payment_eligible")]
    pub is_cc_installment_payment_eligible: bool,
    #[serde(rename = "is_non_cc_installment_payment_eligible")]
    pub is_non_cc_installment_payment_eligible: bool,
    #[serde(rename = "coin_earn_label")]
    pub coin_earn_label: Value,
    #[serde(rename = "show_free_shipping")]
    pub show_free_shipping: bool,
    #[serde(rename = "preview_info")]
    pub preview_info: Value,
    #[serde(rename = "coin_info")]
    pub coin_info: Value,
    #[serde(rename = "exclusive_price_info")]
    pub exclusive_price_info: Value,
    #[serde(rename = "bundle_deal_id")]
    pub bundle_deal_id: i64,
    #[serde(rename = "can_use_bundle_deal")]
    pub can_use_bundle_deal: bool,
    #[serde(rename = "bundle_deal_info")]
    pub bundle_deal_info: Option<BundleDealInfo>,
    #[serde(rename = "is_group_buy_item")]
    pub is_group_buy_item: Value,
    #[serde(rename = "has_group_buy_stock")]
    pub has_group_buy_stock: Value,
    #[serde(rename = "group_buy_info")]
    pub group_buy_info: Value,
    #[serde(rename = "welcome_package_type")]
    pub welcome_package_type: i64,
    #[serde(rename = "welcome_package_info")]
    pub welcome_package_info: Value,
    #[serde(rename = "add_on_deal_info")]
    pub add_on_deal_info: Option<AddOnDealInfo>,
    #[serde(rename = "can_use_wholesale")]
    pub can_use_wholesale: bool,
    #[serde(rename = "is_preferred_plus_seller")]
    pub is_preferred_plus_seller: bool,
    #[serde(rename = "shop_location")]
    pub shop_location: String,
    #[serde(rename = "has_model_with_available_shopee_stock")]
    pub has_model_with_available_shopee_stock: bool,
    #[serde(rename = "voucher_info")]
    pub voucher_info: Value,
    #[serde(rename = "can_use_cod")]
    pub can_use_cod: bool,
    #[serde(rename = "is_on_flash_sale")]
    pub is_on_flash_sale: bool,
    #[serde(rename = "spl_installment_tenure")]
    pub spl_installment_tenure: Value,
    #[serde(rename = "is_live_streaming_price")]
    pub is_live_streaming_price: Value,
    #[serde(rename = "is_mart")]
    pub is_mart: bool,
    #[serde(rename = "pack_size")]
    pub pack_size: Value,
    #[serde(rename = "deep_discount_skin")]
    pub deep_discount_skin: Option<DeepDiscountSkin>,
    #[serde(rename = "is_service_by_shopee")]
    pub is_service_by_shopee: bool,
    #[serde(rename = "spl_repayment_label_repayment")]
    pub spl_repayment_label_repayment: Value,
    #[serde(rename = "spl_repayment_label_text")]
    pub spl_repayment_label_text: Value,
    #[serde(rename = "highlight_video")]
    pub highlight_video: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfoList {
    #[serde(rename = "video_id")]
    pub video_id: String,
    #[serde(rename = "thumb_url")]
    pub thumb_url: String,
    pub duration: i64,
    pub version: i64,
    pub vid: String,
    // pub formats: Value,
    pub formats: Vec<Format>,
    #[serde(rename = "default_format")]
    pub default_format: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub format: i64,
    pub defn: String,
    pub profile: String,
    pub path: String,
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultFormat {
    pub format: i64,
    pub defn: String,
    pub profile: String,
    pub path: String,
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierVariation {
    pub name: String,
    pub options: Vec<String>,
    // pub options: Value,
    #[serde(default)]
    // pub images: Vec<String>,
    pub images: Value,
    pub properties: Vec<Value>,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemRating {
    #[serde(rename = "rating_star")]
    pub rating_star: f64,
    #[serde(rename = "rating_count")]
    pub rating_count: Vec<i64>,
    // pub rating_count: Value,
    #[serde(rename = "rcount_with_context")]
    pub rcount_with_context: i64,
    #[serde(rename = "rcount_with_image")]
    pub rcount_with_image: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleDealInfo {
    #[serde(rename = "bundle_deal_id")]
    pub bundle_deal_id: i64,
    #[serde(rename = "bundle_deal_label")]
    pub bundle_deal_label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOnDealInfo {
    #[serde(rename = "add_on_deal_id")]
    pub add_on_deal_id: i64,
    #[serde(rename = "add_on_deal_label")]
    pub add_on_deal_label: String,
    #[serde(rename = "sub_type")]
    pub sub_type: i64,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepDiscountSkin {
    #[serde(rename = "skin_id")]
    pub skin_id: i64,
    #[serde(rename = "start_time")]
    pub start_time: i64,
    #[serde(rename = "end_time")]
    pub end_time: i64,
    #[serde(rename = "skin_data")]
    pub skin_data: SkinData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinData {
    #[serde(rename = "promo_label")]
    pub promo_label: PromoLabel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoLabel {
    #[serde(rename = "promotion_price")]
    pub promotion_price: String,
    #[serde(rename = "hidden_promotion_price")]
    pub hidden_promotion_price: String,
    pub text: Value,
    #[serde(rename = "start_time")]
    pub start_time: i64,
    #[serde(rename = "end_time")]
    pub end_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingInfo {
    #[serde(rename = "viral_spu_tracking")]
    pub viral_spu_tracking: Value,
    #[serde(rename = "business_tracking")]
    pub business_tracking: Value,
    #[serde(rename = "multi_search_tracking")]
    pub multi_search_tracking: Value,
    pub groupid: i64,
    pub ruleid: Vec<i64>,
    // pub ruleid: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Adjust {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRewrite {
    #[serde(rename = "fe_query_write_status")]
    pub fe_query_write_status: i64,
    #[serde(rename = "rewrite_keyword")]
    pub rewrite_keyword: Value,
    #[serde(rename = "hint_keywords")]
    pub hint_keywords: Value,
    #[serde(rename = "ori_keyword")]
    pub ori_keyword: String,
    #[serde(rename = "ori_total_count")]
    pub ori_total_count: i64,
    #[serde(rename = "rewrite_type")]
    pub rewrite_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowResult {
    pub triggered: bool,
    pub scenarios: Value,
    #[serde(rename = "total_organic_count")]
    pub total_organic_count: i64,
    #[serde(rename = "pre_lrp_total_organic_count")]
    pub pre_lrp_total_organic_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodItemInfo {
    #[serde(rename = "total_count")]
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(rename = "user_type")]
    // pub user_type: Vec<i64>,
    pub user_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experiment {
    pub key: String,
    pub value: String,
}
