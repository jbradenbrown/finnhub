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

/// Filing sentiment analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilingSentiment {
    /// Access number.
    #[serde(rename = "accessNumber")]
    pub access_number: String,
    /// Symbol.
    pub symbol: String,
    /// CIK.
    pub cik: String,
    /// Sentiment scores.
    pub sentiment: SentimentScores,
}

/// Sentiment scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentScores {
    /// Percentage of negative words.
    pub negative: f64,
    /// Percentage of positive words.
    pub positive: f64,
    /// Polarity score.
    pub polarity: f64,
    /// Percentage of litigious words.
    pub litigious: f64,
    /// Percentage of uncertainty words.
    pub uncertainty: f64,
    /// Percentage of constraining words.
    pub constraining: f64,
    /// Percentage of modal-weak words.
    #[serde(rename = "modal-weak")]
    pub modal_weak: f64,
    /// Percentage of modal-strong words.
    #[serde(rename = "modal-strong")]
    pub modal_strong: f64,
    /// Percentage of modal-moderate words.
    #[serde(rename = "modal-moderate")]
    pub modal_moderate: f64,
}

/// Document similarity index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityIndex {
    /// Symbol.
    pub symbol: Option<String>,
    /// CIK.
    pub cik: Option<String>,
    /// Array of similarity data.
    pub similarity: Vec<SimilarityData>,
}

/// Similarity data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityData {
    /// CIK.
    pub cik: String,
    /// Access number of the document.
    #[serde(rename = "accessNumber")]
    pub access_number: String,
    /// Item 1 similarity score.
    pub item1: f64,
    /// Item 2 similarity score.
    pub item2: f64,
    /// Item 1a similarity score.
    pub item1a: f64,
    /// Item 7 similarity score.
    pub item7: f64,
    /// Item 7a similarity score.
    pub item7a: f64,
    /// Form type.
    pub form: String,
    /// Report URL.
    #[serde(rename = "reportUrl")]
    pub report_url: String,
    /// Filing URL.
    #[serde(rename = "filingUrl")]
    pub filing_url: String,
    /// Filed date.
    #[serde(rename = "filedDate")]
    pub filed_date: String,
    /// Accepted date.
    #[serde(rename = "acceptedDate")]
    pub accepted_date: String,
}
