use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub id: String,
    pub name: String,
    pub metric: String,
    pub threshold: f64,
    pub operator: AlertOperator,
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertOperator {
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub config: AlertConfig,
    pub triggered: bool,
    pub last_triggered: Option<i64>,
}

pub struct AlertManager {
    alerts: HashMap<String, Alert>,
}

#[derive(Debug, thiserror::Error)]
pub enum AlertError {
    #[error("Alert not found: {0}")]
    NotFound(String),
    #[error("Alert already exists: {0}")]
    AlreadyExists(String),
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            alerts: HashMap::new(),
        }
    }

    pub fn add_alert(&mut self, config: AlertConfig) -> Result<(), AlertError> {
        if self.alerts.contains_key(&config.id) {
            return Err(AlertError::AlreadyExists(config.id));
        }

        let alert = Alert {
            config,
            triggered: false,
            last_triggered: None,
        };

        self.alerts.insert(alert.config.id.clone(), alert);
        info!("Added alert: {}", alert.config.id);
        Ok(())
    }

    pub fn remove_alert(&mut self, id: &str) -> Result<(), AlertError> {
        if self.alerts.remove(id).is_some() {
            info!("Removed alert: {}", id);
            Ok(())
        } else {
            Err(AlertError::NotFound(id.to_string()))
        }
    }

    pub fn get_alert(&self, id: &str) -> Option<&Alert> {
        self.alerts.get(id)
    }

    pub fn list_alerts(&self) -> Vec<&Alert> {
        self.alerts.values().collect()
    }

    pub fn evaluate_alerts(&mut self, metric_data: &HashMap<String, f64>) {
        for alert in self.alerts.values_mut() {
            if !alert.config.enabled {
                continue;
            }

            if let Some(&value) = metric_data.get(&alert.config.metric) {
                let should_trigger = match alert.config.operator {
                    AlertOperator::GreaterThan => value > alert.config.threshold,
                    AlertOperator::LessThan => value < alert.config.threshold,
                    AlertOperator::EqualTo => (value - alert.config.threshold).abs() < f64::EPSILON,
                };

                if should_trigger && !alert.triggered {
                    alert.triggered = true;
                    alert.last_triggered = Some(chrono::Utc::now().timestamp_millis());
                    self.trigger_alert(alert);
                } else if !should_trigger && alert.triggered {
                    alert.triggered = false;
                    self.resolve_alert(alert);
                }
            }
        }
    }

    fn trigger_alert(&self, alert: &Alert) {
        warn!(
            "ALERT TRIGGERED: {} - Metric '{}' {} {}",
            alert.config.name,
            alert.config.metric,
            match alert.config.operator {
                AlertOperator::GreaterThan => ">",
                AlertOperator::LessThan => "<",
                AlertOperator::EqualTo => "==",
            },
            alert.config.threshold
        );
        // In a full implementation, this would send notifications via email, Slack, etc.
    }

    fn resolve_alert(&self, alert: &Alert) {
        info!("ALERT RESOLVED: {}", alert.config.name);
        // In a full implementation, this would send resolution notifications
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_manager() {
        let mut manager = AlertManager::new();
        
        let config = AlertConfig {
            id: "high_cpu".to_string(),
            name: "High CPU Usage".to_string(),
            metric: "cpu_usage".to_string(),
            threshold: 80.0,
            operator: AlertOperator::GreaterThan,
            enabled: true,
            description: Some("Alert when CPU usage exceeds 80%".to_string()),
        };
        
        assert!(manager.add_alert(config).is_ok());
        assert!(manager.get_alert("high_cpu").is_some());
        
        let alerts = manager.list_alerts();
        assert_eq!(alerts.len(), 1);
    }
    
    #[test]
    fn test_alert_evaluation() {
        let mut manager = AlertManager::new();
        
        let config = AlertConfig {
            id: "high_memory".to_string(),
            name: "High Memory Usage".to_string(),
            metric: "memory_usage_percent".to_string(),
            threshold: 90.0,
            operator: AlertOperator::GreaterThan,
            enabled: true,
            description: None,
        };
        
        manager.add_alert(config).unwrap();
        
        let mut metric_data = HashMap::new();
        metric_data.insert("memory_usage_percent".to_string(), 95.0);
        
        manager.evaluate_alerts(&metric_data);
        
        let alert = manager.get_alert("high_memory").unwrap();
        assert!(alert.triggered);
    }
}