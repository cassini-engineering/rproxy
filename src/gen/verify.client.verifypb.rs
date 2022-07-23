// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyContactRequest {
    #[prost(string, tag="1")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub otp: ::prost::alloc::string::String,
    #[prost(enumeration="ContactType", tag="3")]
    pub r#type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContactType {
    PhoneNumber = 0,
    Email = 1,
}
include!("verify.client.verifypb.serde.rs");
include!("verify.client.verifypb.tonic.rs");
// @@protoc_insertion_point(module)