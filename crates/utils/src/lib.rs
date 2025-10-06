use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn init(&mut self) -> Result<()>;
    fn execute(&self, data: &serde_json::Value) -> Result<serde_json::Value>;
    fn cleanup(&mut self) -> Result<()>;
}

// Plugin manager to handle plugin lifecycle
pub struct PluginManager {
    plugins: HashMap<String, Arc<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Arc<dyn Plugin>) -> Result<()> {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
        Ok(())
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
        self.plugins.get(name)
    }

    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    pub fn execute_plugin(&self, name: &str, data: &serde_json::Value) -> Result<serde_json::Value> {
        if let Some(plugin) = self.plugins.get(name) {
            plugin.execute(data)
        } else {
            Err(anyhow::anyhow!("Plugin not found: {}", name))
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

// Configuration UI structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigOption {
    pub key: String,
    pub label: String,
    pub value: serde_json::Value,
    pub r#type: ConfigType,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ConfigType {
    String,
    Number,
    Boolean,
    Select(Vec<String>),
}

pub struct ConfigManager {
    options: HashMap<String, ConfigOption>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
        }
    }

    pub fn add_option(&mut self, option: ConfigOption) {
        self.options.insert(option.key.clone(), option);
    }

    pub fn get_option(&self, key: &str) -> Option<&ConfigOption> {
        self.options.get(key)
    }

    pub fn set_option_value(&mut self, key: &str, value: serde_json::Value) -> Result<()> {
        if let Some(option) = self.options.get_mut(key) {
            option.value = value;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Config option not found: {}", key))
        }
    }

    pub fn list_options(&self) -> Vec<&ConfigOption> {
        self.options.values().collect()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct TestPlugin {
        name: String,
        version: String,
        description: String,
        executed: Arc<Mutex<bool>>,
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            &self.version
        }

        fn description(&self) -> &str {
            &self.description
        }

        fn init(&mut self) -> Result<()> {
            Ok(())
        }

        fn execute(&self, _data: &serde_json::Value) -> Result<serde_json::Value> {
            *self.executed.lock().unwrap() = true;
            Ok(serde_json::json!({"result": "success"}))
        }

        fn cleanup(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_plugin_manager() -> Result<()> {
        let mut manager = PluginManager::new();
        let executed = Arc::new(Mutex::new(false));
        
        let plugin = Arc::new(TestPlugin {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "A test plugin".to_string(),
            executed: executed.clone(),
        });
        
        manager.register_plugin(plugin)?;
        
        let plugins = manager.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0], "test_plugin");
        
        let data = serde_json::json!({"input": "test"});
        let result = manager.execute_plugin("test_plugin", &data)?;
        assert_eq!(result, serde_json::json!({"result": "success"}));
        assert!(*executed.lock().unwrap());
        
        Ok(())
    }
    
    #[test]
    fn test_config_manager() -> Result<()> {
        let mut manager = ConfigManager::new();
        
        let option = ConfigOption {
            key: "refresh_interval".to_string(),
            label: "Refresh Interval (ms)".to_string(),
            value: serde_json::json!(1000),
            r#type: ConfigType::Number,
            description: Some("How often to refresh metrics".to_string()),
        };
        
        manager.add_option(option);
        
        let options = manager.list_options();
        assert_eq!(options.len(), 1);
        
        let retrieved = manager.get_option("refresh_interval").unwrap();
        assert_eq!(retrieved.value, serde_json::json!(1000));
        
        manager.set_option_value("refresh_interval", serde_json::json!(2000))?;
        let updated = manager.get_option("refresh_interval").unwrap();
        assert_eq!(updated.value, serde_json::json!(2000));
        
        Ok(())
    }
}