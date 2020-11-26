pub mod pubsub;
pub mod service;
pub mod socket;

pub const XPUB_PORT: &'static str = "10240";
pub const XSUB_PORT: &'static str = "10241";
pub const SERV_FRONTEND_PORT: &'static str = "10422";
pub const SERV_BACKEND_PORT: &'static str = "10423";

pub const PUB_PROXY_EP: &'static str = "tcp://localhost:10241";
pub const SUB_PROXY_EP: &'static str = "tcp://localhost:10240";

pub trait RbxMessage: serde::Serialize + serde::de::DeserializeOwned + 'static {}
impl<T> RbxMessage for T where T: serde::Serialize + serde::de::DeserializeOwned + 'static {}
