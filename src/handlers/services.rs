use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;

use hyper::{Body, Request, Response};

use super::authenticator::build_authentication_handler;

pub trait Handles: DynClone {
    fn handle(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error>;
}

clone_trait_object!(Handles);

pub async fn build_handler_map() -> HashMap<String, Box<dyn Handles + Send>> {
    let mut m: HashMap<String, Box<dyn Handles + Send>> = HashMap::new();

    let authenticator_handler = build_authentication_handler()
        .await
        .expect("failed to create authentication handler");

    m.insert(
        "authenticator".to_string(),
        Box::new(authenticator_handler) as Box<dyn Handles + Send>,
    );

    m
}
