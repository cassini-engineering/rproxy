// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloResponse {
    #[prost(string, tag="1")]
    pub greeting: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserRequest {
    #[prost(string, tag="1")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub device_identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    #[prost(string, tag="1")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateRequest {
    #[prost(string, tag="1")]
    pub access_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::super::users::client::userspb::User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTokenRequest {
    #[prost(string, tag="1")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTokenResponse {
    #[prost(string, tag="1")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub refresh_token: ::prost::alloc::string::String,
}
include!("authority.client.authoritypb.serde.rs");
include!("authority.client.authoritypb.tonic.rs");
// @@protoc_insertion_point(module)