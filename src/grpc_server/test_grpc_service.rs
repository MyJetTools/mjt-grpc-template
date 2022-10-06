use super::server::GrpcService;

use my_service_bus_tcp_client::MessageToPublish;
use crate::{{grpc_package | snake_case}}_grpc::{{ grpc_service_name | snake_case }}_server::{{grpc_service_name}};
{% if is_use_sb == "publisher" %}use crate::{TestEventSbContract, TEST_EVENT_SB_TOPIC_NAME, AsBytes};{% endif %}
use crate::{{grpc_package | snake_case}}_grpc::{TestRequest, TestResponse};

#[tonic::async_trait]
impl {{grpc_service_name}} for GrpcService {
    async fn test(
        &self,
        request: tonic::Request<TestRequest>,
    ) -> Result<tonic::Response<TestResponse>, tonic::Status> {
        {% if is_use_telemetry %}let my_telemetry = super::telemetry::get_telemetry(&request.metadata(), request.remote_addr(), "test");{% endif %}

        let request = request.into_inner();
        {% if is_use_sb == "publisher" %}let sb_event = TestEventSbContract{
            test: request.test.clone(),
        };
        self.app.my_sb_connection.publish(TEST_EVENT_SB_TOPIC_NAME, MessageToPublish::new(sb_event.as_bytes())).await.unwrap();{% endif %}

        return Ok(tonic::Response::new(TestResponse { test: request.test }));
    }

}
