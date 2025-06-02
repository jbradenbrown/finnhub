//! ESG and compliance-related models.

use serde::{Deserialize, Serialize};

/// Current ESG score data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGScore {
    /// Symbol.
    pub symbol: String,
    /// Company name.
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    /// ESG Risk Rating.
    #[serde(rename = "ESGRiskRating")]
    pub esg_risk_rating: Option<f64>,
    /// ESG Risk Level.
    #[serde(rename = "ESGRiskLevel")]
    pub esg_risk_level: Option<String>,
    /// Environmental Risk Score.
    #[serde(rename = "environmentRiskScore")]
    pub environment_risk_score: Option<f64>,
    /// Governance Risk Score.
    #[serde(rename = "governanceRiskScore")]
    pub governance_risk_score: Option<f64>,
    /// Social Risk Score.
    #[serde(rename = "socialRiskScore")]
    pub social_risk_score: Option<f64>,
    /// As of date.
    #[serde(rename = "ratingMonth")]
    pub rating_month: Option<String>,
}

/// USPTO patent data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USPTOPatents {
    /// Symbol.
    pub symbol: String,
    /// Array of patent applications.
    pub data: Vec<PatentApplication>,
}

/// Patent application data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatentApplication {
    /// Application number.
    #[serde(rename = "applicationNumber")]
    pub application_number: String,
    /// Company filing with the patent.
    #[serde(rename = "companyFilingName")]
    pub company_filing_name: Vec<String>,
    /// Filing date.
    #[serde(rename = "filingDate")]
    pub filing_date: String,
    /// Publication date.
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,
    /// Patent type.
    #[serde(rename = "patentType")]
    pub patent_type: String,
    /// URL.
    pub url: String,
    /// Patent number.
    #[serde(rename = "patentNumber")]
    pub patent_number: Option<String>,
    /// Filing status.
    #[serde(rename = "filingStatus")]
    pub filing_status: String,
    /// Patent description.
    #[serde(rename = "patentDescription")]
    pub patent_description: Option<String>,
}

/// Visa application data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisaApplications {
    /// Symbol.
    pub symbol: String,
    /// Array of visa applications.
    pub data: Vec<VisaApplication>,
}

/// Visa application details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisaApplication {
    /// Year.
    pub year: i32,
    /// Quarter.
    pub quarter: i32,
    /// Symbol.
    pub symbol: String,
    /// Case number.
    #[serde(rename = "caseNumber")]
    pub case_number: String,
    /// Case status.
    #[serde(rename = "caseStatus")]
    pub case_status: String,
    /// Received date.
    #[serde(rename = "receivedDate")]
    pub received_date: String,
    /// Visa class.
    #[serde(rename = "visaClass")]
    pub visa_class: String,
    /// Job title.
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    /// SOC code.
    #[serde(rename = "socCode")]
    pub soc_code: Option<String>,
    /// Full time position.
    #[serde(rename = "fullTimePosition")]
    pub full_time_position: String,
    /// Begin date.
    #[serde(rename = "beginDate")]
    pub begin_date: String,
    /// End date.
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// Employer name.
    #[serde(rename = "employerName")]
    pub employer_name: String,
    /// Worksite address.
    #[serde(rename = "worksiteAddress")]
    pub worksite_address: Option<String>,
    /// Worksite city.
    #[serde(rename = "worksiteCity")]
    pub worksite_city: Option<String>,
    /// Worksite county.
    #[serde(rename = "worksiteCounty")]
    pub worksite_county: Option<String>,
    /// Worksite state.
    #[serde(rename = "worksiteState")]
    pub worksite_state: Option<String>,
    /// Worksite postal code.
    #[serde(rename = "worksitePostalCode")]
    pub worksite_postal_code: Option<String>,
    /// Wage range from.
    #[serde(rename = "wageRangeFrom")]
    pub wage_range_from: Option<f64>,
    /// Wage range to.
    #[serde(rename = "wageRangeTo")]
    pub wage_range_to: Option<f64>,
    /// Wage unit of pay.
    #[serde(rename = "wageUnitOfPay")]
    pub wage_unit_of_pay: Option<String>,
    /// Wage level.
    #[serde(rename = "wageLevel")]
    pub wage_level: Option<String>,
    /// H1B dependent.
    #[serde(rename = "h1bDependent")]
    pub h1b_dependent: Option<String>,
}

/// Supply chain relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainRelationship {
    /// Symbol.
    pub symbol: Option<String>,
    /// Company name.
    pub name: Option<String>,
    /// Country.
    pub country: Option<String>,
    /// 1-tier supplier.
    #[serde(rename = "oneMonthCorrelation")]
    pub one_month_correlation: Option<f64>,
    /// 1-year correlation.
    #[serde(rename = "oneYearCorrelation")]
    pub one_year_correlation: Option<f64>,
    /// 6-month correlation.
    #[serde(rename = "sixMonthCorrelation")]
    pub six_month_correlation: Option<f64>,
    /// 3-month correlation.
    #[serde(rename = "threeMonthCorrelation")]
    pub three_month_correlation: Option<f64>,
    /// 2-week correlation.
    #[serde(rename = "twoWeekCorrelation")]
    pub two_week_correlation: Option<f64>,
    /// 2-year correlation.
    #[serde(rename = "twoYearCorrelation")]
    pub two_year_correlation: Option<f64>,
}

/// Supply chain data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainData {
    /// Company symbol.
    pub symbol: String,
    /// List of suppliers.
    pub data: Vec<SupplyChainRelationship>,
}

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
