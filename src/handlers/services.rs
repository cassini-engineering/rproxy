use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;

use hyper::{Body, Request, Response};

use super::authenticator::build_authentication_handler;

#[async_trait]
pub trait Handles: DynClone {
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error>;
}

clone_trait_object!(Handles);

pub async fn build_handler_map() -> HashMap<String, Box<dyn Handles + Send + Sync>> {
    let mut m: HashMap<String, Box<dyn Handles + Send + Sync>> = HashMap::new();

    let authenticator_handler = build_authentication_handler()
        .await
        .expect("failed to create authentication handler");

    m.insert(
        "authenticator".to_string().to_owned(),
        Box::new(authenticator_handler) as Box<dyn Handles + Send + Sync>,
    );

    m
}
