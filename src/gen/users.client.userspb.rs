// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email_address: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub verified: bool,
    #[prost(int32, tag="5")]
    pub created_at: i32,
    #[prost(int32, tag="6")]
    pub updated_at: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdRequest {
    #[prost(string, tag="1")]
    pub phone_number: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
    #[prost(string, tag="1")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<User>,
}
include!("users.client.userspb.serde.rs");
include!("users.client.userspb.tonic.rs");
// @@protoc_insertion_point(module)