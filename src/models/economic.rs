//! Economic data models.

use serde::{Deserialize, Serialize};

/// Economic data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicDataPoint {
    /// Date.
    pub date: String,
    /// Value.
    pub value: f64,
}

/// Economic data response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicData {
    /// Economic code.
    pub code: String,
    /// Array of data points.
    pub data: Vec<EconomicDataPoint>,
}

/// Economic indicator code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicCode {
    /// Code.
    pub code: String,
    /// Country.
    pub country: String,
    /// Indicator name.
    pub name: String,
    /// Unit.
    pub unit: String,
}
