use anyhow::Result;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ssh2::Session;
use crate::ssh;

/// Creates a service file
/// ```ini
/// [Unit]
/// Description=
/// StartLimitIntervalSec=0
/// After=network.target
///
/// [Service]
/// Type=exec
/// Restart=always
/// RestartSec=1
/// User=administrator
/// WorkingDirectory=
/// ExecStart=
///
/// [Install]
/// WantedBy=multi-user.target
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServiceFile {
    #[serde(rename = "Unit")]
    pub unit: Unit,
    #[serde(rename = "Service")]
    pub service: Service,
    #[serde(rename = "Install")]
    pub install: Install,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Unit {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "StartLimitIntervalSec")]
    pub start_limit_interval_sec: u64,
    #[serde(rename = "After")]
    pub after: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Service {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Restart")]
    pub restart: String,
    #[serde(rename = "RestartSec")]
    pub restart_sec: u64,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: String,
    #[serde(rename = "ExecStart")]
    pub exec_start: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Install {
    #[serde(rename = "WantedBy")]
    pub wanted_by: String,
}

impl Default for ServiceFile {
    fn default() -> Self {
        ServiceFile {
            unit: Unit {
                description: "".to_string(),
                start_limit_interval_sec: 0,
                after: "network.target".to_string(),
            },
            service: Service {
                type_: "exec".to_string(),
                restart: "always".to_string(),
                restart_sec: 1,
                user: "administrator".to_string(),
                working_directory: "".to_string(),
                exec_start: "".to_string(),
            },
            install: Install {
                wanted_by: "multi-user.target".to_string(),
            },
        }
    }
}
impl ServiceFile {
    pub fn new(name:impl AsRef<str>, working_directory: impl AsRef<str>, executable: impl AsRef<str>)->Self{
        let mut service_file = ServiceFile::default();
        service_file.unit.description = name.as_ref().to_string();
        service_file.service.working_directory = working_directory.as_ref().to_string();
        service_file.service.exec_start = executable.as_ref().to_string();
        service_file
    }
    
    pub fn install(&self, service_name: impl AsRef<str>, ssh_session: &mut Session)->Result<()>{
        let service_name = service_name.as_ref();
        let tmp_file = "tmp.service";
        self.to_file(tmp_file)?;
        
        ssh::upload_file(tmp_file, format!("/etc/systemd/system/{}.service", service_name), ssh_session)?;
        ssh::execute_command("sudo systemctl daemon-reload", ssh_session)?;
        ssh::execute_command(format!("sudo systemctl enable {}", service_name), ssh_session)?;
        
        fs::remove_file(tmp_file)?;
        Ok(())
    }
    
    pub fn to_file(&self, file_path:impl AsRef<Path>)->Result<()>{
        let mut file = File::create(file_path)?;
        file.write_all(self.to_string().as_bytes())?;
        file.flush()?;
        Ok(())
    }
    
}

impl Display for ServiceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let toml_str = toml::to_string(self).unwrap();

        // First, replace quoted string values
        let quoted_string_regex = regex::Regex::new(r#"([a-zA-Z0-9]+) += +"([^"]+)""#).unwrap();
        let unquoted_strings = quoted_string_regex.replace_all(&toml_str, "$1=$2");

        // Then replace numeric values
        let numeric_value_regex = regex::Regex::new(r#"([a-zA-Z0-9]+) += +([0-9]+)"#).unwrap();
        let cleaned_toml = numeric_value_regex.replace_all(&unquoted_strings, "$1=$2");

        write!(f, "{}", cleaned_toml)
    }
}