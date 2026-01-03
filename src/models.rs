use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetric {
    pub name: String,
    pub operation: String,
    pub mean_micros: f64,
    pub min_micros: f64,
    pub max_micros: f64,
    pub std_dev_micros: f64,
    pub iterations: usize,
    pub extra_info: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub signatures: Vec<BenchmarkMetric>,
    pub kem: Vec<BenchmarkMetric>,
    pub keygen_time_secs: f64,
    pub total_time_secs: f64,
}
