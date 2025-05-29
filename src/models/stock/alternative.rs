//! Alternative data models.

use serde::{Deserialize, Serialize};

/// Investment theme portfolio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentTheme {
    /// Theme name.
    pub theme: String,
    /// Array of stocks in the theme.
    pub data: Vec<ThemeStock>,
}

/// Stock in an investment theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeStock {
    /// Stock symbol.
    pub symbol: String,
}

/// Earnings call transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallTranscript {
    /// Company symbol.
    pub symbol: String,
    /// Transcript ID.
    pub id: String,
    /// Title.
    pub title: String,
    /// Time of the event.
    pub time: String,
    /// Year.
    pub year: i32,
    /// Quarter.
    pub quarter: i32,
    /// Audio link.
    pub audio: Option<String>,
    /// Array of participants.
    pub participant: Vec<TranscriptParticipant>,
    /// Transcript content.
    pub transcript: Vec<TranscriptContent>,
}

/// Transcript participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptParticipant {
    /// Participant name.
    pub name: String,
    /// Participant description/title.
    pub description: String,
}

/// Transcript content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptContent {
    /// Speaker name.
    pub name: String,
    /// Array of speech segments.
    pub speech: Vec<String>,
}

/// Earnings call transcripts list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallTranscriptsList {
    /// Company symbol.
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
    /// Time of the event.
    pub time: String,
    /// Year.
    pub year: i32,
    /// Quarter.
    pub quarter: i32,
    /// Symbol.
    pub symbol: String,
}

/// Earnings call live events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallLive {
    /// Array of live earnings call events.
    pub event: Vec<EarningsCallLiveEvent>,
}

/// Earnings call live event details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCallLiveEvent {
    /// Symbol.
    pub symbol: String,
    /// Event name.
    pub event: String,
    /// Date time in UTC.
    pub time: String,
    /// Year.
    pub year: i32,
    /// Quarter.
    pub quarter: i32,
    /// Live audio URL.
    #[serde(rename = "liveAudio")]
    pub live_audio: Option<String>,
    /// Recording URL.
    pub recording: Option<String>,
}

/// Investor presentations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorPresentations {
    /// Symbol.
    pub symbol: String,
    /// Array of presentations.
    pub data: Vec<Presentation>,
}

/// Presentation details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presentation {
    /// Event.
    pub event: String,
    /// Presentation date.
    pub date: String,
    /// URL.
    pub url: String,
    /// Title.
    pub title: Option<String>,
    /// Description.
    pub description: Option<String>,
}