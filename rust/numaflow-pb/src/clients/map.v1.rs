// This file is @generated by prost-build.
/// *
/// MapRequest represents a request element.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapRequest {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<map_request::Request>,
    /// This ID is used to uniquely identify a map request
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub handshake: ::core::option::Option<Handshake>,
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<TransmissionStatus>,
}
/// Nested message and enum types in `MapRequest`.
pub mod map_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bytes = "vec", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, optional, tag = "3")]
        pub event_time: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag = "4")]
        pub watermark: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(map = "string, string", tag = "5")]
        pub headers: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
///
/// Handshake message between client and server to indicate the start of transmission.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Handshake {
    /// Required field indicating the start of transmission.
    #[prost(bool, tag = "1")]
    pub sot: bool,
}
///
/// Status message to indicate the status of the message.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TransmissionStatus {
    #[prost(bool, tag = "1")]
    pub eot: bool,
}
/// *
/// MapResponse represents a response element.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<map_response::Result>,
    /// This ID is used to refer the responses to the request it corresponds to.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub handshake: ::core::option::Option<Handshake>,
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<TransmissionStatus>,
}
/// Nested message and enum types in `MapResponse`.
pub mod map_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(string, repeated, tag = "1")]
        pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bytes = "vec", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(string, repeated, tag = "3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// *
/// ReadyResponse is the health check result.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
/// Generated client implementations.
pub mod map_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MapClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MapClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MapClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MapClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            MapClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// MapFn applies a function to each map request element.
        pub async fn map_fn(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MapRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MapResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/map.v1.Map/MapFn");
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new("map.v1.Map", "MapFn"));
            self.inner.streaming(req, path, codec).await
        }
        /// IsReady is the heartbeat endpoint for gRPC.
        pub async fn is_ready(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::ReadyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/map.v1.Map/IsReady");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("map.v1.Map", "IsReady"));
            self.inner.unary(req, path, codec).await
        }
    }
}