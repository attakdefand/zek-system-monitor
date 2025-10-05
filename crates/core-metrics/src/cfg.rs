use serde::Deserialize; use std::fs;
#[derive(Debug, Clone, Deserialize)] pub struct Config{pub refresh:Refresh,pub collectors:Collectors,pub exporters:Exporters,pub web:Web}
#[derive(Debug, Clone, Deserialize)] pub struct Refresh{pub interval_ms:u64}
#[derive(Debug, Clone, Deserialize, Default)] pub struct Collectors{pub cpu:bool,pub mem:bool,pub load:bool}
#[derive(Debug, Clone, Deserialize, Default)] pub struct Exporters{#[serde(default)] pub prometheus: Option<PromConfig>, #[serde(skip)] pub prometheus_bind: Option<String>}
#[derive(Debug, Clone, Deserialize)] pub struct PromConfig{pub bind:String}
#[derive(Debug, Clone, Deserialize, Default)] pub struct Web{pub bind: Option<String>}
pub fn load_cfg(p:&str)->anyhow::Result<Config>{let raw=fs::read_to_string(p)?; let mut c:Config=toml::from_str(&raw)?; c.exporters.prometheus_bind=c.exporters.prometheus.as_ref().map(|x|x.bind.clone()); Ok(c)}
