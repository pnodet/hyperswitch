use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use common_utils::id_type;

use crate::enums::{Currency, RefundStatus};

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
)]
// TODO RefundType api_models_oss need to mapped to storage_model
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RefundType {
    InstantRefund,
    RegularRefund,
    RetryRefund,
}

use super::{ForexMetric, NameDescription, TimeRange};
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct RefundFilters {
    #[serde(default)]
    pub currency: Vec<Currency>,
    #[serde(default)]
    pub refund_status: Vec<RefundStatus>,
    #[serde(default)]
    pub connector: Vec<String>,
    #[serde(default)]
    pub refund_type: Vec<RefundType>,
    #[serde(default)]
    pub profile_id: Vec<id_type::ProfileId>,
    #[serde(default)]
    pub refund_reason: Vec<String>,
    #[serde(default)]
    pub refund_error_message: Vec<String>,
}

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    strum::AsRefStr,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    strum::Display,
    strum::EnumIter,
    Clone,
    Copy,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RefundDimensions {
    Currency,
    RefundStatus,
    Connector,
    RefundType,
    ProfileId,
    RefundReason,
    RefundErrorMessage,
}

#[derive(
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumIter,
    strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RefundMetrics {
    RefundSuccessRate,
    RefundCount,
    RefundSuccessCount,
    RefundProcessedAmount,
    SessionizedRefundSuccessRate,
    SessionizedRefundCount,
    SessionizedRefundSuccessCount,
    SessionizedRefundProcessedAmount,
    SessionizedRefundReason,
    SessionizedRefundErrorMessage,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct ReasonsResult {
    pub reason: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct ErrorMessagesResult {
    pub error_message: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumIter,
    strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RefundDistributions {
    #[strum(serialize = "refund_reason")]
    SessionizedRefundReason,
    #[strum(serialize = "refund_error_message")]
    SessionizedRefundErrorMessage,
}
impl ForexMetric for RefundMetrics {
    fn is_forex_metric(&self) -> bool {
        matches!(
            self,
            Self::RefundProcessedAmount | Self::SessionizedRefundProcessedAmount
        )
    }
}

pub mod metric_behaviour {
    pub struct RefundSuccessRate;
    pub struct RefundCount;
    pub struct RefundSuccessCount;
    pub struct RefundProcessedAmount;
}

impl From<RefundMetrics> for NameDescription {
    fn from(value: RefundMetrics) -> Self {
        Self {
            name: value.to_string(),
            desc: String::new(),
        }
    }
}

impl From<RefundDimensions> for NameDescription {
    fn from(value: RefundDimensions) -> Self {
        Self {
            name: value.to_string(),
            desc: String::new(),
        }
    }
}

#[derive(Debug, serde::Serialize, Eq)]
pub struct RefundMetricsBucketIdentifier {
    pub currency: Option<Currency>,
    pub refund_status: Option<String>,
    pub connector: Option<String>,
    pub refund_type: Option<String>,
    pub profile_id: Option<String>,
    pub refund_reason: Option<String>,
    pub refund_error_message: Option<String>,
    #[serde(rename = "time_range")]
    pub time_bucket: TimeRange,
    #[serde(rename = "time_bucket")]
    #[serde(with = "common_utils::custom_serde::iso8601custom")]
    pub start_time: time::PrimitiveDateTime,
}

impl Hash for RefundMetricsBucketIdentifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.currency.hash(state);
        self.refund_status.hash(state);
        self.connector.hash(state);
        self.refund_type.hash(state);
        self.profile_id.hash(state);
        self.refund_reason.hash(state);
        self.refund_error_message.hash(state);
        self.time_bucket.hash(state);
    }
}
impl PartialEq for RefundMetricsBucketIdentifier {
    fn eq(&self, other: &Self) -> bool {
        let mut left = DefaultHasher::new();
        self.hash(&mut left);
        let mut right = DefaultHasher::new();
        other.hash(&mut right);
        left.finish() == right.finish()
    }
}

impl RefundMetricsBucketIdentifier {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        currency: Option<Currency>,
        refund_status: Option<String>,
        connector: Option<String>,
        refund_type: Option<String>,
        profile_id: Option<String>,
        refund_reason: Option<String>,
        refund_error_message: Option<String>,
        normalized_time_range: TimeRange,
    ) -> Self {
        Self {
            currency,
            refund_status,
            connector,
            refund_type,
            profile_id,
            refund_reason,
            refund_error_message,
            time_bucket: normalized_time_range,
            start_time: normalized_time_range.start_time,
        }
    }
}
#[derive(Debug, serde::Serialize)]
pub struct RefundMetricsBucketValue {
    pub successful_refunds: Option<u32>,
    pub total_refunds: Option<u32>,
    pub refund_success_rate: Option<f64>,
    pub refund_count: Option<u64>,
    pub refund_success_count: Option<u64>,
    pub refund_processed_amount: Option<u64>,
    pub refund_processed_amount_in_usd: Option<u64>,
    pub refund_processed_count: Option<u64>,
    pub refund_reason_distribution: Option<Vec<ReasonsResult>>,
    pub refund_error_message_distribution: Option<Vec<ErrorMessagesResult>>,
    pub refund_reason_count: Option<u64>,
    pub refund_error_message_count: Option<u64>,
}
#[derive(Debug, serde::Serialize)]
pub struct RefundMetricsBucketResponse {
    #[serde(flatten)]
    pub values: RefundMetricsBucketValue,
    #[serde(flatten)]
    pub dimensions: RefundMetricsBucketIdentifier,
}
