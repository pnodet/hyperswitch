use std::convert::From;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::configs;

#[derive(Default, Clone, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = configs)]
pub struct ConfigNew {
    pub key: String,
    pub config: String,
}

#[derive(Default, Clone, Debug, Identifiable, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = configs, primary_key(key), check_for_backend(diesel::pg::Pg))]
pub struct Config {
    pub key: String,
    pub config: String,
}

#[derive(Debug)]
pub enum ConfigUpdate {
    Update { config: Option<String> },
}

#[derive(Clone, Debug, AsChangeset, Default)]
#[diesel(table_name = configs)]
pub struct ConfigUpdateInternal {
    config: Option<String>,
}

impl ConfigUpdateInternal {
    pub fn create_config(self, source: Config) -> Config {
        Config { ..source }
    }
}

impl From<ConfigUpdate> for ConfigUpdateInternal {
    fn from(config_update: ConfigUpdate) -> Self {
        match config_update {
            ConfigUpdate::Update { config } => Self { config },
        }
    }
}

impl From<ConfigNew> for Config {
    fn from(config_new: ConfigNew) -> Self {
        Self {
            key: config_new.key,
            config: config_new.config,
        }
    }
}
