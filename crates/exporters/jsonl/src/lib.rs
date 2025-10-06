use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub file_path: String,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    CSV,
    JSON,
    JSONL, // JSON Lines format
}

pub struct DataExporter {
    config: ExportConfig,
    writer: BufWriter<File>,
}

impl DataExporter {
    pub fn new(config: ExportConfig) -> Result<Self> {
        let file = File::create(&config.file_path)?;
        let writer = BufWriter::new(file);
        
        Ok(Self { config, writer })
    }
    
    pub fn export_data<T: Serialize>(&mut self, data: &[T]) -> Result<()> {
        match self.config.format {
            ExportFormat::CSV => self.export_csv(data),
            ExportFormat::JSON => self.export_json(data),
            ExportFormat::JSONL => self.export_jsonl(data),
        }
    }
    
    fn export_csv<T: Serialize>(&mut self, data: &[T]) -> Result<()> {
        // For simplicity, we'll just serialize to JSON and write as CSV-like
        writeln!(self.writer, "data")?;
        for item in data {
            let json = serde_json::to_string(item)?;
            writeln!(self.writer, "{}", json)?;
        }
        self.writer.flush()?;
        Ok(())
    }
    
    fn export_json<T: Serialize>(&mut self, data: &[T]) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        writeln!(self.writer, "{}", json)?;
        self.writer.flush()?;
        Ok(())
    }
    
    fn export_jsonl<T: Serialize>(&mut self, data: &[T]) -> Result<()> {
        for item in data {
            let json = serde_json::to_string(item)?;
            writeln!(self.writer, "{}", json)?;
        }
        self.writer.flush()?;
        Ok(())
    }
    
    pub fn finish(mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    
    #[derive(Serialize, Deserialize, Debug)]
    struct TestMetric {
        timestamp: i64,
        metric_name: String,
        value: f64,
    }
    
    #[test]
    fn test_json_export() -> Result<()> {
        let config = ExportConfig {
            format: ExportFormat::JSON,
            file_path: "test_export.json".to_string(),
            metrics: vec!["cpu".to_string(), "memory".to_string()],
        };
        
        let mut exporter = DataExporter::new(config)?;
        
        let data = vec![
            TestMetric {
                timestamp: 1234567890,
                metric_name: "cpu".to_string(),
                value: 45.5,
            },
            TestMetric {
                timestamp: 1234567891,
                metric_name: "memory".to_string(),
                value: 65.2,
            }
        ];
        
        exporter.export_data(&data)?;
        exporter.finish()?;
        
        // Clean up
        std::fs::remove_file("test_export.json")?;
        
        Ok(())
    }
}