//! SEC filings and document models.

use serde::{Deserialize, Serialize};

/// SEC filing data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filing {
    /// Access number.
    #[serde(rename = "accessNumber")]
    pub access_number: Option<String>,
    /// Symbol.
    pub symbol: Option<String>,
    /// CIK.
    pub cik: Option<String>,
    /// Form type.
    pub form: Option<String>,
    /// Filed date.
    #[serde(rename = "filedDate")]
    pub filed_date: Option<String>,
    /// Accepted date.
    #[serde(rename = "acceptedDate")]
    pub accepted_date: Option<String>,
    /// Report's URL.
    #[serde(rename = "reportUrl")]
    pub report_url: Option<String>,
    /// Filing's URL.
    #[serde(rename = "filingUrl")]
    pub filing_url: Option<String>,
}

/// International filing data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalFiling {
    /// Symbol.
    pub symbol: String,
    /// Company name.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Filing date.
    #[serde(rename = "filedDate")]
    pub filed_date: String,
    /// Filing category.
    pub category: String,
    /// Filing title.
    pub title: String,
    /// Filing description.
    pub description: Option<String>,
    /// Filing URL.
    pub url: String,
    /// Filing language.
    pub language: String,
    /// Filing country.
    pub country: String,
}

/// Earnings call transcript data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallTranscript {
    /// Symbol.
    pub symbol: String,
    /// Transcript data.
    pub transcript: Vec<TranscriptSegment>,
    /// Participant list.
    pub participant: Vec<TranscriptParticipant>,
    /// Audio link.
    pub audio: String,
    /// Transcript ID.
    pub id: String,
    /// Title.
    pub title: String,
    /// Time.
    pub time: String,
    /// Year.
    pub year: i32,
    /// Quarter.
    pub quarter: i32,
}

/// Transcript segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    /// Speaker name.
    pub name: String,
    /// Speaker position.
    pub position: String,
    /// Start time.
    #[serde(rename = "startTime")]
    pub start_time: i32,
    /// Speech content.
    pub speech: String,
}

/// Transcript participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptParticipant {
    /// Participant name.
    pub name: String,
    /// Participant description.
    pub description: String,
    /// Participant role.
    pub role: String,
}

/// Earnings call transcripts list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallTranscriptsList {
    /// Symbol.
    pub symbol: String,
    /// Array of transcript metadata.
    pub transcripts: Vec<TranscriptMetadata>,
}

/// Transcript metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptMetadata {
    /// Transcript ID.
    pub id: String,
    /// Title.
    pub title: String,
    /// Time.
    pub time: String,
    /// Year.
    pub year: i32,
    /// Quarter.
    pub quarter: i32,
}

/// Earnings call live events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallLive {
    /// List of events.
    pub events: Vec<EarningsCallLiveEvent>,
}

/// Earnings call live event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallLiveEvent {
    /// Symbol.
    pub symbol: String,
    /// Event date.
    #[serde(rename = "eventDate")]
    pub event_date: String,
    /// Start time.
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// Audio link.
    pub audio: String,
    /// Company name.
    #[serde(rename = "companyName")]
    pub company_name: String,
    /// Event name.
    #[serde(rename = "eventName")]
    pub event_name: String,
}

/// Investor presentations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorPresentations {
    /// Symbol.
    pub symbol: String,
    /// Array of presentations.
    pub presentations: Vec<InvestorPresentation>,
}

/// Investor presentation data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorPresentation {
    /// Date.
    pub date: String,
    /// Title.
    pub title: String,
    /// URL.
    pub url: String,
}

/// Document similarity index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityIndex {
    /// CIK.
    pub cik: String,
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