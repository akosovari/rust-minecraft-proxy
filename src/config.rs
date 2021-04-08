use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    listen_addr: String,
    unknown_host: BTreeMap<String, String>,
    hosts: BTreeMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut unknown_host: BTreeMap<String, String> = BTreeMap::new();
        unknown_host.insert("kick_message".to_string(), "§cInvalid Address".to_string());
        unknown_host.insert("motd".to_string(), "§cUnknown host!\n§7Please use a valid address to connect.".to_string());
        unknown_host.insert("protocol_name".to_string(), "§crust-minecraft-proxy".to_string());
        Self {
            listen_addr: "0.0.0.0:25565".to_string(),
            unknown_host,
            hosts: BTreeMap::new(),
        }
    }
}

impl Config {
    pub fn load_or_init(path: &Path) -> Config {
        if path.exists() {
            toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
        } else {
            info!("Configuration file does not exist. Use defaults.");
            let default = Config::default();
            trace!("Default configuration: {:?}", default);
            let string = toml::to_string(&default).unwrap();
            fs::write(path, &string).unwrap();
            default
        }
    }

    pub fn get_unknown_host_kick_msg(&self) -> String {
        let mut message: String =  "{\"text\":\"".to_owned();
        message.push_str(&self.unknown_host.get("kick_message").as_deref().unwrap_or(&"§cInvalid Address".to_string()));
        message.push_str("\"}");
        message
    }

    pub fn get_unknown_host_motd(&self) -> String {
        let mut motd: String = "{\"version\": {\"name\": \"".to_owned();
        motd.push_str(&self.unknown_host.get("protocol_name").as_deref().unwrap_or(&"§cInvalid Address".to_string()));
        motd.push_str("\", \"protocol\": -1 }, \"players\": {\"max\": 0, \"online\": 0, \"sample\": [] }, \"description\": { \"text\": \"");
        motd.push_str(&self.unknown_host.get("motd").as_deref().unwrap_or(&"§cUnknown host!\n§7Please use a valid address to connect.".to_string()));
        motd.push_str("\" }}");
        motd
    }

    pub fn get_listen_addr(&self) -> &str {
        &self.listen_addr
    }

    pub fn get_hosts(&self) -> &BTreeMap<String, String> {
        &self.hosts
    }

    pub fn get_addr_by_host(&self, host: &str) -> Option<&String> {
        self.hosts.get(host)
    }
}
