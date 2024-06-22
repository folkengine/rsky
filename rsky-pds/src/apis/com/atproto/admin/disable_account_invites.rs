use crate::account_manager::AccountManager;
use crate::auth_verifier::Moderator;
use crate::models::{InternalErrorCode, InternalErrorMessageResponse};
use anyhow::Result;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rsky_lexicon::com::atproto::admin::DisableAccountInvitesInput;

#[rocket::post(
    "/xrpc/com.atproto.admin.disableAccountInvites",
    format = "json",
    data = "<body>"
)]
pub async fn disable_account_invites(
    body: Json<DisableAccountInvitesInput>,
    _auth: Moderator,
) -> Result<(), status::Custom<Json<InternalErrorMessageResponse>>> {
    let DisableAccountInvitesInput { account, .. } = body.into_inner();
    match AccountManager::set_account_invites_disabled(&account, true).await {
        Ok(_) => Ok(()),
        Err(error) => {
            let internal_error = InternalErrorMessageResponse {
                code: Some(InternalErrorCode::InternalError),
                message: Some(error.to_string()),
            };
            return Err(status::Custom(
                Status::InternalServerError,
                Json(internal_error),
            ));
        }
    }
}