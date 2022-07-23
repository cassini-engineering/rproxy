use hyper::{Body, Response};
use tonic::transport::Channel;

use crate::gen::authority::client::authoritypb::authority_service_client::AuthorityServiceClient;
use crate::gen::users::client::userspb::users_service_client::UsersServiceClient;
use crate::gen::verify::client::verifypb::verify_service_client::VerifyServiceClient;

use super::services::Handles;

#[derive(Clone)]
pub struct AuthenticatorHandler {
    authority_client: AuthorityServiceClient<Channel>,
    users_client: UsersServiceClient<Channel>,
    verify_client: VerifyServiceClient<Channel>,
}

pub async fn build_authentication_handler() -> Result<AuthenticatorHandler, String> {
    let dst: &str = "http://localhost:8000";

    let ac = AuthorityServiceClient::connect(dst)
        .await
        .expect("failed to connect to authority service");

    let vc = VerifyServiceClient::connect(dst)
        .await
        .expect("failed to connect to verify service");

    let uc = UsersServiceClient::connect(dst)
        .await
        .expect("failed to connect to users service");

    let ret = AuthenticatorHandler {
        authority_client: ac,
        verify_client: vc,
        users_client: uc,
    };

    return Ok(ret);
}

impl Handles for AuthenticatorHandler {
    fn handle(&self, req: hyper::Request<hyper::Body>) -> Result<Response<Body>, hyper::Error> {
        Ok(hyper::Response::new(hyper::Body::from("200")))
    }
}
