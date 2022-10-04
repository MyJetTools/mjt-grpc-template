use std::sync::Arc;

use my_postgres::{MyPostgres, MyPostgressError};
use my_telemetry::MyTelemetryContext;

use crate::APP_NAME;

use super::TestDataDbModel;

const TABLE_NAME: &str = "test_data";

pub struct TestDataRepository {
    postgress: MyPostgres,
}


impl TestDataRepository {
    pub async fn new(settings_reader: Arc<crate::settings::SettingsReader>) -> Self {
        let postgress = MyPostgres::new(
            APP_NAME.to_string(),
            settings_reader.clone(),
            my_logger::LOGGER.clone(),
        )
        .await;

        Self { postgress }
    }

    pub async fn register(
        &self,
        id: String,
        name: String,
        telemetry_context: MyTelemetryContext,
    ) -> Result<TestDataDbModel, MyPostgressError> {
        let register_order_dto = TestDataDbModel {
            id,
            name
        };

        self.postgress
            .insert_db_entity(&register_order_dto, TABLE_NAME, Some(telemetry_context))
            .await?;

        Ok(register_order_dto)
    }
}
