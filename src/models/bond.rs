//! Bond-related data models.

use serde::{Deserialize, Serialize};

/// Bond profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondProfile {
    /// ISIN.
    pub isin: Option<String>,
    /// CUSIP.
    pub cusip: Option<String>,
    /// FIGI.
    pub figi: Option<String>,
    /// Coupon rate.
    pub coupon: Option<f64>,
    /// Maturity date.
    #[serde(rename = "maturityDate")]
    pub maturity_date: Option<String>,
    /// Offering price.
    #[serde(rename = "offeringPrice")]
    pub offering_price: Option<f64>,
    /// Issue date.
    #[serde(rename = "issueDate")]
    pub issue_date: Option<String>,
    /// Bond type.
    #[serde(rename = "bondType")]
    pub bond_type: Option<String>,
    /// Debt type.
    #[serde(rename = "debtType")]
    pub debt_type: Option<String>,
    /// Industry group.
    #[serde(rename = "industryGroup")]
    pub industry_group: Option<String>,
    /// Industry sub-group.
    #[serde(rename = "industrySubGroup")]
    pub industry_sub_group: Option<String>,
    /// Asset.
    pub asset: Option<String>,
    /// Asset type.
    #[serde(rename = "assetType")]
    pub asset_type: Option<String>,
    /// Dated date.
    #[serde(rename = "datedDate")]
    pub dated_date: Option<String>,
    /// First coupon date.
    #[serde(rename = "firstCouponDate")]
    pub first_coupon_date: Option<String>,
    /// Original offering amount.
    #[serde(rename = "originalOffering")]
    pub original_offering: Option<f64>,
    /// Outstanding amount.
    #[serde(rename = "amountOutstanding")]
    pub amount_outstanding: Option<f64>,
    /// Payment frequency.
    #[serde(rename = "paymentFrequency")]
    pub payment_frequency: Option<String>,
    /// Security level.
    #[serde(rename = "securityLevel")]
    pub security_level: Option<String>,
    /// Whether the bond is callable.
    pub callable: Option<bool>,
    /// Coupon type.
    #[serde(rename = "couponType")]
    pub coupon_type: Option<String>,
}

/// Bond price data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondPrice {
    /// Symbol.
    pub symbol: Option<String>,
    /// Current price.
    #[serde(rename = "c")]
    pub current_price: Option<f64>,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
}

/// Bond tick data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondTickData {
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Number of ticks skipped.
    pub skip: i64,
    /// Number of ticks returned.
    pub count: i64,
    /// Total number of ticks for that date.
    pub total: i64,
    /// List of volume data.
    #[serde(rename = "v")]
    pub volume: Vec<f64>,
    /// List of price data.
    #[serde(rename = "p")]
    pub price: Vec<f64>,
    /// List of timestamp in UNIX ms.
    #[serde(rename = "t")]
    pub timestamp: Vec<i64>,
    /// List of venues/exchanges.
    #[serde(rename = "x")]
    pub exchange: Vec<String>,
    /// List of trade conditions.
    #[serde(rename = "c")]
    pub conditions: Option<Vec<Vec<String>>>,
}

/// Yield curve data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldCurvePoint {
    /// Date.
    #[serde(rename = "d")]
    pub date: String,
    /// Yield value.
    #[serde(rename = "v")]
    pub value: f64,
}

/// Bond yield curve response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondYieldCurve {
    /// Yield curve code.
    pub code: String,
    /// Array of yield curve data points.
    pub data: Vec<YieldCurvePoint>,
}