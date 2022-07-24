use async_trait::async_trait;
use hyper::body;
use hyper::{Body, Method, Response, StatusCode};
use tonic::transport::Channel;

use super::services::Handles;
use crate::gen::authority::client::authoritypb::authority_service_client::AuthorityServiceClient;
use crate::gen::authority::client::authoritypb::RegisterUserRequest;
use crate::gen::typespb::Empty;
use crate::gen::users::client::userspb::users_service_client::UsersServiceClient;
use crate::gen::verify::client::verifypb::verify_service_client::VerifyServiceClient;

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

#[async_trait]
impl Handles for AuthenticatorHandler {
    async fn handle(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            // Hello
            (&Method::GET, "/authenticator/hello") => {
                let r = Empty {};
                let res = self
                    .clone()
                    .authority_client
                    .hello(tonic::Request::new(r))
                    .await;
                match res {
                    Ok(r) => Ok(hyper::Response::new(Body::from(
                        serde_json::json!(r.into_inner()).to_string(),
                    ))),
                    Err(_e) => {
                        let mut r = hyper::Response::default();
                        *r.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
                        Ok(r)
                    }
                }
            }

            (&Method::POST, "/authenticator/register") => {
                let b = body::to_bytes(req.into_body()).await?;

                let bs = String::from_utf8(b.to_vec()).expect("request was not utf-8");
                let r: RegisterUserRequest = serde_json::from_str(bs.as_str())
                    .expect("failed to parse register user request");

                let res = self
                    .clone()
                    .authority_client
                    .register_user(tonic::Request::new(r))
                    .await
                    .unwrap();

                return Ok(Response::new(Body::from(
                    serde_json::json!(res.into_inner()).to_string(),
                )));
            }

            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }
}
