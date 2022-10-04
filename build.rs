fn main() {
    tonic_build::compile_protos("proto/{{grpc_service_name}}.proto").unwrap();
}