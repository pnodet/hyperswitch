use actix_web::{web, HttpRequest, HttpResponse};
use router_env::{instrument, tracing, Flow};

use super::app::AppState;
use crate::{
    core::{api_locking, poll},
    services::{api, authentication as auth},
    types::{api::PollId, domain},
};

#[cfg(feature = "v1")]
/// Poll - Retrieve Poll Status
#[utoipa::path(
    get,
    path = "/poll/status/{poll_id}",
    params(
        ("poll_id" = String, Path, description = "The identifier for poll")
    ),
    responses(
        (status = 200, description = "The poll status was retrieved successfully", body = PollResponse),
        (status = 404, description = "Poll not found")
    ),
    tag = "Poll",
    operation_id = "Retrieve Poll Status",
    security(("publishable_key" = []))
)]
#[instrument(skip_all, fields(flow = ?Flow::RetrievePollStatus))]
pub async fn retrieve_poll_status(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::RetrievePollStatus;
    let poll_id = PollId {
        poll_id: path.into_inner(),
    };
    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        poll_id,
        |state, auth, req, _| {
            let merchant_context = domain::MerchantContext::NormalMerchant(Box::new(
                domain::Context(auth.merchant_account, auth.key_store),
            ));
            poll::retrieve_poll_status(state, req, merchant_context)
        },
        &auth::HeaderAuth(auth::PublishableKeyAuth),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}
