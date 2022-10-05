use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    {% if is_use_psql %}#[serde(rename = "PostgresConnString")]
    pub postgres_conn_string: String,{% endif %}
    {% if is_seq_enabled %}#[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,{% endif %}
    {% if is_use_telemetry %}#[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,{% endif %}
    {% if is_use_nosql == "both" %}#[serde(rename = "MyNoSqlDataWriter")]
    pub my_no_sql_data_writer: String,
    #[serde(rename = "MyNoSqlDataReader")]
    pub my_no_sql_data_reader: String,{% endif %}
    {% if is_use_nosql == "writer" %}#[serde(rename = "MyNoSqlDataWriter")]
    pub my_no_sql_data_writer: String,{% endif %}
    {% if is_use_nosql == "reader" %}#[serde(rename = "MyNoSqlDataReader")]
    pub my_no_sql_data_reader: String,{% endif %}
}

{% if is_seq_enabled %}#[async_trait::async_trait]
impl my_seq_logger::SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.seq_conn_string.clone()
    }
}{% endif %}

{% if is_use_psql %}#[async_trait::async_trait]
impl my_postgres::PostgressSettings for SettingsReader {
    async fn get_connection_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.postgres_conn_string.clone()
    }
}{% endif %}

{% if is_use_telemetry %}#[async_trait::async_trait]
impl my_telemetry_writer::MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_telemetry.clone()
    }
}{% endif %}
