// @generated
/// Generated client implementations.
pub mod zitadel_actions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ZitadelActionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ZitadelActionsClient<tonic::transport::Channel> {
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
    impl<T> ZitadelActionsClient<T>
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
        ) -> ZitadelActionsClient<InterceptedService<T, F>>
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
            ZitadelActionsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_target(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/CreateTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "CreateTarget",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn patch_target(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/PatchTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "PatchTarget",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_target(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/DeleteTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "DeleteTarget",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Target by ID

 Returns the target identified by the requested ID.
*/
        pub async fn get_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/GetTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "GetTarget",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Search targets

 Search all matching targets. By default all targets of the instance are returned.
 Make sure to include a limit and sorting for pagination.
*/
        pub async fn search_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchTargetsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/SearchTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "SearchTargets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Sets an execution to call a target or include the targets of another execution.

 Setting an empty list of targets will remove all targets from the execution, making it a noop.
*/
        pub async fn set_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::SetExecutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetExecutionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/SetExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "SetExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Search executions

 Search all matching executions. By default all executions of the instance are returned that have at least one execution target.
 Make sure to include a limit and sorting for pagination.
*/
        pub async fn search_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchExecutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchExecutionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/SearchExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "SearchExecutions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List all available functions

 List all available functions which can be used as condition for executions.
*/
        pub async fn list_execution_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionFunctionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionFunctionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/ListExecutionFunctions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "ListExecutionFunctions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List all available methods

 List all available methods which can be used as condition for executions.
*/
        pub async fn list_execution_methods(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionMethodsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionMethodsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/ListExecutionMethods",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "ListExecutionMethods",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_execution_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionServicesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.action.v3alpha.ZITADELActions/ListExecutionServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.action.v3alpha.ZITADELActions",
                        "ListExecutionServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
