use serde::{Deserialize, Serialize};

///Config used by refinery-cli
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub main: Main,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ConfigDbType {
    Mysql,
    Postgres,
    Sqlite,
}

impl Config {
    pub fn new(db_type: ConfigDbType, db_path: &str) -> Config {
        Config {
            main: Main {
                db_type,
                db_path: db_path.into(),
                db_user: None,
                db_pass: None,
                db_name: None,
            },
        }
    }

    pub fn set_db_user(self, db_user: &str) -> Config {
        Config {
            main: Main {
                db_user: Some(db_user.into()),
                ..self.main
            },
        }
    }

    pub fn set_db_pass(self, db_pass: &str) -> Config {
        Config {
            main: Main {
                db_pass: Some(db_pass.into()),
                ..self.main
            },
        }
    }

    pub fn set_db_name(self, db_name: &str) -> Config {
        Config {
            main: Main {
                db_name: Some(db_name.into()),
                ..self.main
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    pub db_type: ConfigDbType,
    pub db_path: String,
    pub db_user: Option<String>,
    pub db_pass: Option<String>,
    pub db_name: Option<String>,
}
