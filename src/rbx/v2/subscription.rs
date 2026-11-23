use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::UniverseId, util::QueryString};

use crate::rbx::http::{self, HttpClient};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SubscriptionView {
    Basic,
    Full,
}

impl std::fmt::Display for SubscriptionView {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Basic => "BASIC",
                Self::Full => "FULL",
            }
        )
    }
}

pub struct GetSubscriptionParams {
    pub universe_id: UniverseId,
    pub subscription_product: String,
    pub subscription: String,
    pub view: Option<SubscriptionView>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSubscriptionResponse {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub active: bool,
    pub will_renew: bool,
    pub last_billing_time: String,
    pub next_renew_time: String,
    pub expire_time: String,
    pub state: SubscriptionState,
    pub expiration_details: SubscriptionExpirationDetails,
    pub purchase_platform: SubscriptionPurchasePlatform,
    pub payment_provider: SubscriptionPaymentProvider,
    pub user: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionExpirationDetails {
    pub reason: SubscriptionExpirationReason,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionState {
    StateUnspecified,
    SubscribedWillRenew,
    SubscribedWillNotRenew,
    SubscribedRenewalPaymentPending,
    Expired,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionExpirationReason {
    ExpirationReasonUnspecified,
    ProductInactive,
    ProductDeleted,
    SubscriberCancelled,
    SubscriberRefunded,
    Lapsed,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPurchasePlatform {
    PurchasePlatformUnspecified,
    Desktop,
    Mobile,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPaymentProvider {
    PaymentProviderUnspecified,
    Stripe,
    Apple,
    Google,
    RobloxCredit,
}

pub async fn get_subscription(
    http: &HttpClient,
    params: &GetSubscriptionParams,
) -> Result<GetSubscriptionResponse, Error> {
    let url = http::url(&format!(
        "/cloud/v2/universes/{universeId}/subscription-products/{product}/subscriptions/{subscription}",
        universeId = params.universe_id,
        product = params.subscription_product,
        subscription = params.subscription,
    ));

    let mut query: QueryString = vec![];
    if let Some(view) = &params.view {
        query.push(("view", view.to_string()));
    }

    let res = http.get(url).query(&query).send().await?;

    http::json::<GetSubscriptionResponse>(res).await
}
