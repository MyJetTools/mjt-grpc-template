mod app_ctx;
mod grpc_server;
{% if is_use_psql %}mod db_repositories;{% endif %}
{% if is_use_nosql != "no" %}mod nosql;{% endif %}
{% if is_use_sb != "no" %}mod sb;{% endif %}
mod settings;

pub mod {{grpc_package | snake_case}}_grpc {
    tonic::include_proto!("{{grpc_package | snake_case}}");
}

pub use app_ctx::*;
pub use grpc_server::*;
{% if is_use_psql %}pub use db_repositories::*;{% endif %}
{% if is_use_nosql != "no" %}pub use nosql::*;{% endif %}
{% if is_use_sb != "no" %}pub use sb::*;{% endif %}
pub use settings::*;