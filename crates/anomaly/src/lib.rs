use serde::{Deserialize, Serialize};
use core_metrics::collectors::snapshot::Snapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub metric: String,
    pub trend: Trend,
    pub confidence: f64,
    pub prediction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trend {
    Increasing,
    Decreasing,
    Stable,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub metric: String,
    pub value: f64,
    pub threshold: f64,
    pub timestamp: i64,
    pub description: String,
}

pub struct TrendAnalyzer;

impl TrendAnalyzer {
    pub fn analyze_cpu_trend(snapshots: &[Snapshot]) -> TrendAnalysis {
        if snapshots.len() < 2 {
            return TrendAnalysis {
                metric: "cpu_usage".to_string(),
                trend: Trend::Unknown,
                confidence: 0.0,
                prediction: None,
            };
        }

        let values: Vec<f64> = snapshots
            .iter()
            .map(|s| s.cpu_total_pct)
            .collect();

        let trend = Self::calculate_trend(&values);
        let confidence = Self::calculate_confidence(&values);
        let prediction = Self::predict_next(&values);

        TrendAnalysis {
            metric: "cpu_usage".to_string(),
            trend,
            confidence,
            prediction,
        }
    }

    pub fn analyze_memory_trend(snapshots: &[Snapshot]) -> TrendAnalysis {
        if snapshots.len() < 2 {
            return TrendAnalysis {
                metric: "memory_usage".to_string(),
                trend: Trend::Unknown,
                confidence: 0.0,
                prediction: None,
            };
        }

        let values: Vec<f64> = snapshots
            .iter()
            .map(|s| (s.mem_used_bytes as f64) / (s.mem_total_bytes as f64) * 100.0)
            .collect();

        let trend = Self::calculate_trend(&values);
        let confidence = Self::calculate_confidence(&values);
        let prediction = Self::predict_next(&values);

        TrendAnalysis {
            metric: "memory_usage".to_string(),
            trend,
            confidence,
            prediction,
        }
    }

    fn calculate_trend(values: &[f64]) -> Trend {
        if values.len() < 2 {
            return Trend::Unknown;
        }

        let first = values[0];
        let last = values[values.len() - 1];
        let diff = last - first;
        let threshold = 5.0; // 5% threshold for trend detection

        if diff > threshold {
            Trend::Increasing
        } else if diff < -threshold {
            Trend::Decreasing
        } else {
            Trend::Stable
        }
    }

    fn calculate_confidence(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        // Simple confidence based on data point count
        (values.len() as f64 / 10.0).min(1.0)
    }

    fn predict_next(values: &[f64]) -> Option<f64> {
        if values.len() < 2 {
            return None;
        }

        // Simple linear prediction based on last two points
        let len = values.len();
        let last = values[len - 1];
        let prev = values[len - 2];
        let diff = last - prev;
        Some(last + diff)
    }
}

pub struct AnomalyDetector;

impl AnomalyDetector {
    pub fn detect_cpu_anomalies(snapshots: &[Snapshot]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        
        for snapshot in snapshots {
            // Check for high CPU usage (> 90%)
            if snapshot.cpu_total_pct > 90.0 {
                anomalies.push(Anomaly {
                    metric: "cpu_usage".to_string(),
                    value: snapshot.cpu_total_pct,
                    threshold: 90.0,
                    timestamp: snapshot.ts,
                    description: "High CPU usage detected".to_string(),
                });
            }
            
            // Check for very low CPU usage (< 5%) when we expect activity
            if snapshot.cpu_total_pct < 5.0 {
                anomalies.push(Anomaly {
                    metric: "cpu_usage".to_string(),
                    value: snapshot.cpu_total_pct,
                    threshold: 5.0,
                    timestamp: snapshot.ts,
                    description: "Very low CPU usage detected".to_string(),
                });
            }
        }
        
        anomalies
    }
    
    pub fn detect_memory_anomalies(snapshots: &[Snapshot]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        
        for snapshot in snapshots {
            let memory_usage_percent = (snapshot.mem_used_bytes as f64) / (snapshot.mem_total_bytes as f64) * 100.0;
            
            // Check for high memory usage (> 90%)
            if memory_usage_percent > 90.0 {
                anomalies.push(Anomaly {
                    metric: "memory_usage".to_string(),
                    value: memory_usage_percent,
                    threshold: 90.0,
                    timestamp: snapshot.ts,
                    description: "High memory usage detected".to_string(),
                });
            }
        }
        
        anomalies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_metrics::collectors::snapshot::Snapshot;
    
    #[test]
    fn test_trend_analysis() {
        let snapshots = vec![
            create_test_snapshot(1000, 45.0, 1000, 2000),
            create_test_snapshot(2000, 50.0, 1000, 2000),
            create_test_snapshot(3000, 55.0, 1000, 2000),
        ];
        
        let cpu_trend = TrendAnalyzer::analyze_cpu_trend(&snapshots);
        assert!(matches!(cpu_trend.trend, Trend::Increasing));
        
        let memory_trend = TrendAnalyzer::analyze_memory_trend(&snapshots);
        assert!(matches!(memory_trend.trend, Trend::Stable));
    }
    
    #[test]
    fn test_anomaly_detection() {
        let snapshots = vec![
            create_test_snapshot(1000, 95.0, 1000, 2000), // High CPU
            create_test_snapshot(2000, 45.0, 1900, 2000), // High memory (95% usage)
        ];
        
        let cpu_anomalies = AnomalyDetector::detect_cpu_anomalies(&snapshots);
        assert_eq!(cpu_anomalies.len(), 1);
        assert_eq!(cpu_anomalies[0].value, 95.0);
        
        let memory_anomalies = AnomalyDetector::detect_memory_anomalies(&snapshots);
        assert_eq!(memory_anomalies.len(), 1);
        assert!(memory_anomalies[0].value > 90.0);
    }
    
    fn create_test_snapshot(ts: i64, cpu_pct: f64, mem_used: u64, mem_total: u64) -> Snapshot {
        Snapshot {
            ts,
            cpu_total_pct: cpu_pct,
            cpu_per_core: vec![cpu_pct as f32],
            mem_used_bytes: mem_used,
            mem_total_bytes: mem_total,
            swap_used_bytes: 0,
            swap_total_bytes: 0,
            load1: 0.0,
            load5: 0.0,
            load15: 0.0,
            network: vec![],
            disks: vec![],
            top_processes: vec![],
            sensors: vec![],
            batteries: vec![],
            gpus: vec![],
            connections: vec![],
            process_tree: vec![],
            containers: vec![],
        }
    }
}