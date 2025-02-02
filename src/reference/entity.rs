use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub metrics: ReferenceMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceMetrics {
    pub uses: u32
}
