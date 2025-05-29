//! Executive and corporate governance models.

use serde::{Deserialize, Serialize};

/// Executive or board member information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executive {
    /// Executive name.
    pub name: Option<String>,
    /// Age.
    pub age: Option<i64>,
    /// Title.
    pub title: Option<String>,
    /// Year first appointed.
    pub since: Option<String>,
    /// Sex.
    pub sex: Option<String>,
    /// Total compensation.
    pub compensation: Option<i64>,
    /// Compensation currency.
    pub currency: Option<String>,
}

/// Company executives response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyExecutives {
    /// Company symbol.
    pub symbol: String,
    /// Array of executives and board members.
    pub executive: Vec<Executive>,
}

/// Congressional trading data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressionalTrade {
    /// Symbol.
    pub symbol: String,
    /// Transaction date.
    #[serde(rename = "transactionDate")]
    pub transaction_date: String,
    /// Transaction amount.
    #[serde(rename = "transactionAmount")]
    pub transaction_amount: String,
    /// Name.
    pub name: String,
    /// Owned by.
    #[serde(rename = "ownedBy")]
    pub owned_by: String,
    /// Position.
    pub position: String,
    /// Asset name.
    #[serde(rename = "assetName")]
    pub asset_name: Option<String>,
    /// Filing date.
    #[serde(rename = "filingDate")]
    pub filing_date: Option<String>,
}

/// Congressional trading response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressionalTrading {
    /// Symbol.
    pub symbol: String,
    /// Array of congressional trades.
    pub data: Vec<CongressionalTrade>,
}

/// Lobbying data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyingData {
    /// Symbol.
    pub symbol: String,
    /// Name.
    pub name: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Country.
    pub country: Option<String>,
    /// Year.
    pub year: i32,
    /// Period.
    pub period: String,
    /// Income.
    pub income: f64,
    /// Expenses.
    pub expenses: f64,
    /// Client ID.
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    /// Registrant ID.
    #[serde(rename = "registrantId")]
    pub registrant_id: Option<String>,
}

/// Lobbying response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lobbying {
    /// Symbol.
    pub symbol: String,
    /// Array of lobbying data.
    pub data: Vec<LobbyingData>,
}

/// USA spending data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USASpendingData {
    /// Symbol.
    pub symbol: String,
    /// Recipient name.
    #[serde(rename = "recipientName")]
    pub recipient_name: Option<String>,
    /// Recipient parent name.
    #[serde(rename = "recipientParentName")]
    pub recipient_parent_name: Option<String>,
    /// Award description.
    #[serde(rename = "awardDescription")]
    pub award_description: Option<String>,
    /// Country.
    pub country: Option<String>,
    /// Action date.
    #[serde(rename = "actionDate")]
    pub action_date: String,
    /// Total value.
    #[serde(rename = "totalValue")]
    pub total_value: f64,
    /// Performance start date.
    #[serde(rename = "performanceStartDate")]
    pub performance_start_date: Option<String>,
    /// Performance end date.
    #[serde(rename = "performanceEndDate")]
    pub performance_end_date: Option<String>,
    /// Awarding agency name.
    #[serde(rename = "awardingAgencyName")]
    pub awarding_agency_name: Option<String>,
    /// Awarding sub agency name.
    #[serde(rename = "awardingSubAgencyName")]
    pub awarding_sub_agency_name: Option<String>,
    /// Awarding office name.
    #[serde(rename = "awardingOfficeName")]
    pub awarding_office_name: Option<String>,
    /// Performance country.
    #[serde(rename = "performanceCountry")]
    pub performance_country: Option<String>,
    /// Performance city.
    #[serde(rename = "performanceCity")]
    pub performance_city: Option<String>,
    /// Performance county.
    #[serde(rename = "performanceCounty")]
    pub performance_county: Option<String>,
    /// Performance state.
    #[serde(rename = "performanceState")]
    pub performance_state: Option<String>,
    /// Performance zip code.
    #[serde(rename = "performanceZipCode")]
    pub performance_zip_code: Option<String>,
    /// Award type.
    #[serde(rename = "awardType")]
    pub award_type: Option<String>,
    /// NAICS code.
    #[serde(rename = "naicsCode")]
    pub naics_code: Option<String>,
}

/// USA spending response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USASpending {
    /// Symbol.
    pub symbol: String,
    /// Array of USA spending data.
    pub data: Vec<USASpendingData>,
}