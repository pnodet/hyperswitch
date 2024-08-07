use common_utils::pii;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

use crate::{encryption::Encryption, schema::business_profile};

#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Identifiable,
    Queryable,
    Selectable,
    router_derive::DebugAsDisplay,
)]
#[diesel(table_name = business_profile, primary_key(profile_id), check_for_backend(diesel::pg::Pg))]
pub struct BusinessProfile {
    pub profile_id: String,
    pub merchant_id: String,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<serde_json::Value>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: bool,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<serde_json::Value>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<serde_json::Value>,
    pub payout_link_config: Option<serde_json::Value>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
}

#[derive(Clone, Debug, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile, primary_key(profile_id))]
pub struct BusinessProfileNew {
    pub profile_id: String,
    pub merchant_id: String,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<serde_json::Value>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: bool,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<serde_json::Value>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<serde_json::Value>,
    pub payout_link_config: Option<serde_json::Value>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
}

#[derive(Clone, Debug, Default, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile)]
pub struct BusinessProfileUpdateInternal {
    pub profile_name: Option<String>,
    pub modified_at: Option<time::PrimitiveDateTime>,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub webhook_details: Option<serde_json::Value>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: Option<bool>,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<serde_json::Value>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<serde_json::Value>,
    pub payout_link_config: Option<serde_json::Value>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BusinessProfileUpdate {
    Update {
        profile_name: Option<String>,
        modified_at: Option<time::PrimitiveDateTime>,
        return_url: Option<String>,
        enable_payment_response_hash: Option<bool>,
        payment_response_hash_key: Option<String>,
        redirect_to_merchant_with_http_post: Option<bool>,
        webhook_details: Option<serde_json::Value>,
        metadata: Option<pii::SecretSerdeValue>,
        routing_algorithm: Option<serde_json::Value>,
        intent_fulfillment_time: Option<i64>,
        frm_routing_algorithm: Option<serde_json::Value>,
        payout_routing_algorithm: Option<serde_json::Value>,
        is_recon_enabled: Option<bool>,
        applepay_verified_domains: Option<Vec<String>>,
        payment_link_config: Option<serde_json::Value>,
        session_expiry: Option<i64>,
        authentication_connector_details: Option<serde_json::Value>,
        payout_link_config: Option<serde_json::Value>,
        extended_card_info_config: Option<pii::SecretSerdeValue>,
        use_billing_as_payment_method_billing: Option<bool>,
        collect_shipping_details_from_wallet_connector: Option<bool>,
        collect_billing_details_from_wallet_connector: Option<bool>,
        is_connector_agnostic_mit_enabled: Option<bool>,
        outgoing_webhook_custom_http_headers: Option<Encryption>,
    },
    ExtendedCardInfoUpdate {
        is_extended_card_info_enabled: Option<bool>,
    },
    ConnectorAgnosticMitUpdate {
        is_connector_agnostic_mit_enabled: Option<bool>,
    },
}

impl From<BusinessProfileUpdate> for BusinessProfileUpdateInternal {
    fn from(business_profile_update: BusinessProfileUpdate) -> Self {
        match business_profile_update {
            BusinessProfileUpdate::Update {
                profile_name,
                modified_at,
                return_url,
                enable_payment_response_hash,
                payment_response_hash_key,
                redirect_to_merchant_with_http_post,
                webhook_details,
                metadata,
                routing_algorithm,
                intent_fulfillment_time,
                frm_routing_algorithm,
                payout_routing_algorithm,
                is_recon_enabled,
                applepay_verified_domains,
                payment_link_config,
                session_expiry,
                authentication_connector_details,
                payout_link_config,
                extended_card_info_config,
                use_billing_as_payment_method_billing,
                collect_shipping_details_from_wallet_connector,
                collect_billing_details_from_wallet_connector,
                is_connector_agnostic_mit_enabled,
                outgoing_webhook_custom_http_headers,
            } => Self {
                profile_name,
                modified_at,
                return_url,
                enable_payment_response_hash,
                payment_response_hash_key,
                redirect_to_merchant_with_http_post,
                webhook_details,
                metadata,
                routing_algorithm,
                intent_fulfillment_time,
                frm_routing_algorithm,
                payout_routing_algorithm,
                is_recon_enabled,
                applepay_verified_domains,
                payment_link_config,
                session_expiry,
                authentication_connector_details,
                payout_link_config,
                extended_card_info_config,
                use_billing_as_payment_method_billing,
                collect_shipping_details_from_wallet_connector,
                collect_billing_details_from_wallet_connector,
                is_connector_agnostic_mit_enabled,
                outgoing_webhook_custom_http_headers,
                ..Default::default()
            },
            BusinessProfileUpdate::ExtendedCardInfoUpdate {
                is_extended_card_info_enabled,
            } => Self {
                is_extended_card_info_enabled,
                ..Default::default()
            },
            BusinessProfileUpdate::ConnectorAgnosticMitUpdate {
                is_connector_agnostic_mit_enabled,
            } => Self {
                is_connector_agnostic_mit_enabled,
                ..Default::default()
            },
        }
    }
}

impl From<BusinessProfileNew> for BusinessProfile {
    fn from(new: BusinessProfileNew) -> Self {
        Self {
            profile_id: new.profile_id,
            merchant_id: new.merchant_id,
            profile_name: new.profile_name,
            created_at: new.created_at,
            modified_at: new.modified_at,
            return_url: new.return_url,
            enable_payment_response_hash: new.enable_payment_response_hash,
            payment_response_hash_key: new.payment_response_hash_key,
            redirect_to_merchant_with_http_post: new.redirect_to_merchant_with_http_post,
            webhook_details: new.webhook_details,
            metadata: new.metadata,
            routing_algorithm: new.routing_algorithm,
            intent_fulfillment_time: new.intent_fulfillment_time,
            frm_routing_algorithm: new.frm_routing_algorithm,
            payout_routing_algorithm: new.payout_routing_algorithm,
            is_recon_enabled: new.is_recon_enabled,
            applepay_verified_domains: new.applepay_verified_domains,
            payment_link_config: new.payment_link_config,
            session_expiry: new.session_expiry,
            authentication_connector_details: new.authentication_connector_details,
            payout_link_config: new.payout_link_config,
            is_connector_agnostic_mit_enabled: new.is_connector_agnostic_mit_enabled,
            is_extended_card_info_enabled: new.is_extended_card_info_enabled,
            extended_card_info_config: new.extended_card_info_config,
            use_billing_as_payment_method_billing: new.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: new
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: new
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: new.outgoing_webhook_custom_http_headers,
        }
    }
}

impl BusinessProfileUpdate {
    pub fn apply_changeset(self, source: BusinessProfile) -> BusinessProfile {
        let BusinessProfileUpdateInternal {
            profile_name,
            modified_at: _,
            return_url,
            enable_payment_response_hash,
            payment_response_hash_key,
            redirect_to_merchant_with_http_post,
            webhook_details,
            metadata,
            routing_algorithm,
            intent_fulfillment_time,
            frm_routing_algorithm,
            payout_routing_algorithm,
            is_recon_enabled,
            applepay_verified_domains,
            payment_link_config,
            session_expiry,
            authentication_connector_details,
            payout_link_config,
            is_extended_card_info_enabled,
            extended_card_info_config,
            is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers,
        } = self.into();
        BusinessProfile {
            profile_name: profile_name.unwrap_or(source.profile_name),
            modified_at: common_utils::date_time::now(),
            return_url,
            enable_payment_response_hash: enable_payment_response_hash
                .unwrap_or(source.enable_payment_response_hash),
            payment_response_hash_key,
            redirect_to_merchant_with_http_post: redirect_to_merchant_with_http_post
                .unwrap_or(source.redirect_to_merchant_with_http_post),
            webhook_details,
            metadata,
            routing_algorithm,
            intent_fulfillment_time,
            frm_routing_algorithm,
            payout_routing_algorithm,
            is_recon_enabled: is_recon_enabled.unwrap_or(source.is_recon_enabled),
            applepay_verified_domains,
            payment_link_config,
            session_expiry,
            authentication_connector_details,
            payout_link_config,
            is_extended_card_info_enabled,
            is_connector_agnostic_mit_enabled,
            extended_card_info_config,
            use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers,
            ..source
        }
    }
}
