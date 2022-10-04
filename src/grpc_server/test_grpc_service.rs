use super::server::GrpcService;
use crate::{{grpc_package}}_grpc::{TestRequest, TestResponse};
use crate::{{grpc_package}}_grpc::{{ grpc_service_name | snake_case }}_server::{{grpc_service_name}};

#[tonic::async_trait]
impl {{grpc_service_name}} for GrpcService {
    async fn test(
        &self,
        request: tonic::Request<TestRequest>,
    ) -> Result<tonic::Response<TestResponse>, tonic::Status> {
        {% if is_use_telemetry %}let my_telemetry = super::telemetry::get_telemetry(&request.metadata(), request.remote_addr(), "test");{% endif %}

        let request = request.into_inner();

        return Ok(tonic::Response::new(TestResponse { test: request.test }));
    }

}
