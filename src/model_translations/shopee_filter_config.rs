use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopeeFilterConfig {
    #[serde(rename = "bff_meta")]
    pub bff_meta: Value,
    pub error: Value,
    #[serde(rename = "error_msg")]
    pub error_msg: Value,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "filter_configuration")]
    pub filter_configuration: FilterConfiguration,
    #[serde(rename = "sub_errs")]
    pub sub_errs: Value,
    #[serde(rename = "bff_ab_test_sign")]
    pub bff_ab_test_sign: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterConfiguration {
    pub filters: Vec<Filter>,
    // pub filters: Value,
    #[serde(rename = "filter_groups")]
    // pub filter_groups: Value,
    pub filter_groups: Vec<FilterGroup>,
    #[serde(rename = "filter_shortcut_groups")]
    // pub filter_shortcut_groups: Value,
    pub filter_shortcut_groups: Vec<FilterShortcutGroup>,
    #[serde(rename = "dynamic_filter_group_data")]
    pub dynamic_filter_group_data: DynamicFilterGroupData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "label_id")]
    pub label_id: Option<i64>,
    #[serde(rename = "is_dynamic")]
    // pub is_dynamic: bool,
    pub is_dynamic: Value,
    pub translations: Vec<Translation>,
    // pub translations: Value,
    #[serde(rename = "icon_url")]
    pub icon_url: Value,
    #[serde(rename = "curation_id")]
    pub curation_id: Value,
    #[serde(rename = "global_attr_id")]
    pub global_attr_id: Value,
    #[serde(rename = "item_tag_id")]
    pub item_tag_id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    #[serde(rename = "language_code")]
    pub language_code: String,
    pub text: String,
    #[serde(rename = "short_text")]
    pub short_text: Option<String>,
    #[serde(rename = "is_default")]
    // pub is_default: bool,
    pub is_default: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterGroup {
    pub name: String,
    // pub filters: Value,
    pub filters: Vec<Filter2>,
    pub logic: i64,
    // pub hidden: bool,
    pub hidden: Value,
    // pub translations: Value,
    pub translations: Vec<Translation2>,
    #[serde(rename = "collapse_settings")]
    pub collapse_settings: Option<CollapseSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter2 {
    pub name: String,
    // pub hidden: Option<bool>,
    pub hidden: Option<Value>,
    #[serde(rename = "schedule_settings")]
    pub schedule_settings: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation2 {
    #[serde(rename = "language_code")]
    pub language_code: String,
    pub text: String,
    #[serde(rename = "short_text")]
    pub short_text: Value,
    #[serde(rename = "is_default")]
    // pub is_default: bool,
    pub is_default: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollapseSettings {
    #[serde(rename = "start_offset")]
    pub start_offset: i64,
    #[serde(rename = "display_limit")]
    pub display_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterShortcutGroup {
    #[serde(rename = "is_default")]
    // pub is_default: bool,
    pub is_default: Value,
    // pub filters: Value,
    pub filters: Vec<Filter3>,
    #[serde(rename = "schedule_settings")]
    pub schedule_settings: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter3 {
    pub name: String,
    // pub hidden: bool,
    pub hidden: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFilterGroupData {
    pub facets: Value,
    // pub facets: Vec<Facet>,
    // pub locations: Value,
    pub locations: Vec<Location>,
    pub brands: Value,
    // pub brands: Vec<Brand>,
    // pub shippings: Value,
    pub shippings: Vec<Shipping>,
    #[serde(rename = "price_ranges")]
    pub price_ranges: Value,
    // #[serde(rename = "filter_algorithm")]
    // pub filter_algorithm: String,
    pub attributes: Value,
    #[serde(rename = "cc_installment_info")]
    pub cc_installment_info: CcInstallmentInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facet {
    pub category: Category,
    pub catid: i64,
    pub count: i64,
    #[serde(rename = "show_parent_category")]
    pub show_parent_category: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub parentids: Vec<i64>,
    // pub parentids: Value,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "is_default_subcat")]
    pub is_default_subcat: Value,
    #[serde(rename = "parent_category_detail")]
    pub parent_category_detail: Option<ParentCategoryDetail>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentCategoryDetail {
    pub category: Category2,
    pub catid: i64,
    pub count: i64,
    #[serde(rename = "show_parent_category")]
    pub show_parent_category: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    pub parentids: Value,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "is_default_subcat")]
    pub is_default_subcat: Value,
    #[serde(rename = "parent_category_detail")]
    pub parent_category_detail: Value,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
    pub level: String,
    #[serde(rename = "sub_location")]
    pub sub_location: Value,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub translations: Value,
    #[serde(rename = "tag_ids")]
    // pub tag_ids: Vec<i64>,
    pub tag_ids: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Brand {
    pub brandid: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipping {
    pub positionid: i64,
    pub name: String,
    pub channelids: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "item_tag_ids")]
    pub item_tag_ids: Vec<i64>,
    // pub item_tag_ids: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CcInstallmentInfo {
    #[serde(rename = "is_cc_installment_payment_eligible")]
    pub is_cc_installment_payment_eligible: Value,
    #[serde(rename = "is_non_cc_installment_payment_eligible")]
    pub is_non_cc_installment_payment_eligible: Value,
}
