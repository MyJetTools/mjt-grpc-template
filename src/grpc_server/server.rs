use crate::AppContext;
use crate::{{grpc_package}}_grpc::{{ grpc_service_name | snake_case }}_server::{{grpc_service_name}}Server;

use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;

#[derive(Clone)]

pub struct GrpcService {
    pub app: Arc<AppContext>,
}

impl GrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

pub fn start_grpc_server(app: Arc<AppContext>, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = GrpcService::new(app);

    println!("Listening to {:?} as grpc endpoint", addr);

    tokio::spawn(async move {
        anyhow::Context::context(
            Server::builder()
                .add_service({{grpc_service_name}}Server::new(service.clone()))
                .serve(addr)
                .await,
            "Server error",
        )
        .unwrap();
    });
}
