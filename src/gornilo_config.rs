use std::{collections::HashMap, env, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GorniloConfig {
    pub project: ProjectConfig,
    pub vet_flags: VetFlagsConfig,
    pub collections: HashMap<String, String>,
    pub testing: TestingConfig,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct VetFlagsConfig {
    pub warnings_as_errors: bool,
    pub unused_variables: bool,
    pub unused_imports: bool,
    pub tabs: bool,
    pub style: bool,
    pub semicolon: bool,
    pub cast: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TestingConfig {
    pub source_path: Option<String>,
    pub all_packages: bool,
}

impl GorniloConfig {
    #[inline]
    pub fn default(project_name: &str) -> GorniloConfig {
        GorniloConfig {
            project: ProjectConfig {
                name: project_name.to_string(),
            },
            vet_flags: VetFlagsConfig {
                tabs: true,
                cast: true,
                style: true,
                semicolon: true,
                unused_imports: true,
                unused_variables: true,
                warnings_as_errors: true,
            },
            collections: HashMap::new(),
            testing: TestingConfig {
                source_path: None,
                all_packages: false,
            },
        }
    }
}

#[inline]
pub fn parse_config() -> GorniloConfig {
    let config_path = env::current_dir()
        .expect("Failed to get the current directory")
        .join("gornilo.toml");

    let mut config_raw = String::new();

    File::open(config_path)
        .expect("Failed to open gornilo config")
        .read_to_string(&mut config_raw)
        .expect("Failed to parse config file to string");

    toml::from_str(&config_raw).expect("Failed to parse config file")
}
