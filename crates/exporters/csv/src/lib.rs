use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;

pub struct CsvExporter {
    writer: csv::Writer<BufWriter<File>>,
}

impl CsvExporter {
    pub fn new(file_path: &str) -> Result<Self> {
        let file = File::create(file_path)?;
        let writer = csv::Writer::from_writer(BufWriter::new(file));
        Ok(Self { writer })
    }

    pub fn write_record<T: Serialize>(&mut self, record: &T) -> Result<()> {
        self.writer.serialize(record)?;
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

    #[derive(Serialize, Deserialize)]
    struct TestRecord {
        name: String,
        value: f64,
        timestamp: i64,
    }

    #[test]
    fn test_csv_export() -> Result<()> {
        let mut exporter = CsvExporter::new("test.csv")?;
        
        let record = TestRecord {
            name: "cpu_usage".to_string(),
            value: 45.5,
            timestamp: 1234567890,
        };
        
        exporter.write_record(&record)?;
        exporter.finish()?;
        
        // Verify the file was created
        assert!(std::path::Path::new("test.csv").exists());
        
        // Clean up
        std::fs::remove_file("test.csv")?;
        
        Ok(())
    }
}