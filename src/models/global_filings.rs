//! Global filings models (filter, search, search-in-filing).

use serde::{Deserialize, Serialize};

/// Body for the `/global-filings/search` POST endpoint.
///
/// Build with `Default::default()` and set the fields you need; all are
/// optional. List-style fields (e.g. `symbols`, `forms`) accept up to 50
/// comma-separated values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchBody {
    /// Search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Comma-separated ISINs (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isins: Option<String>,
    /// Comma-separated CUSIPs (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusips: Option<String>,
    /// Comma-separated SEC CIKs (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciks: Option<String>,
    /// Comma-separated SEDAR issuer numbers (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sedar_ids: Option<String>,
    /// Comma-separated Companies House numbers (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_ids: Option<String>,
    /// Comma-separated symbols (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Comma-separated SEDOLs (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sedols: Option<String>,
    /// Comma-separated sources (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Comma-separated form types (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forms: Option<String>,
    /// Comma-separated GICS codes (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gics: Option<String>,
    /// Comma-separated NAICS codes (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics: Option<String>,
    /// Comma-separated exhibit types (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exhibits: Option<String>,
    /// Comma-separated exchanges (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchanges: Option<String>,
    /// Comma-separated countries (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<String>,
    /// Comma-separated SEC exchanges acts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acts: Option<String>,
    /// Comma-separated market-cap buckets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps: Option<String>,
    /// From date in `YYYY-MM-DD` format (default: last week).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// To date in `YYYY-MM-DD` format (default: today).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Page number (default 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Sort order (default `sortMostRecent`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Whether to return only highlighted excerpts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted: Option<bool>,
}

/// Body for the `/global-filings/search-in-filing` POST endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InFilingSearchBody {
    /// Search query.
    pub query: String,
    /// Filing ID to search within.
    pub filing_id: String,
}

/// One search filter option (e.g. one form type, one source).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    /// Filter id (use this value in the matching field of the search body).
    pub id: Option<String>,
    /// Display name.
    pub name: Option<String>,
}

/// One filing matched by a search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilingResponse {
    /// Filing ID.
    pub filing_id: Option<String>,
    /// Filing title.
    pub title: Option<String>,
    /// Filer entity ID.
    pub filer_id: Option<String>,
    /// Symbols associated with the filing.
    #[serde(default)]
    pub symbol: Option<serde_json::Value>,
    /// Filer name.
    pub name: Option<String>,
    /// Acceptance date.
    pub acceptance_date: Option<String>,
    /// Filed date.
    pub filed_date: Option<String>,
    /// Report date.
    pub report_date: Option<String>,
    /// Form type.
    pub form: Option<String>,
    /// Whether this is an amendment.
    pub amend: Option<bool>,
    /// Source identifier.
    pub source: Option<String>,
    /// Number of pages.
    pub page_count: Option<i64>,
    /// Number of documents.
    pub document_count: Option<i64>,
}

/// Response from `/global-filings/search`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Total filings matching the criteria.
    pub count: Option<i64>,
    /// Server time spent on the query (ms).
    pub took: Option<i64>,
    /// Current page.
    pub page: Option<i64>,
    /// Matching filings.
    #[serde(default)]
    pub filings: Vec<FilingResponse>,
}

/// One excerpt within a document, when highlighting is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcerptResponse {
    /// Excerpt content.
    pub content: Option<String>,
    /// Snippet ID.
    pub snippet_id: Option<String>,
    /// Start offset within the document.
    pub start_offset: Option<String>,
    /// End offset within the document.
    pub end_offset: Option<String>,
}

/// One document within a filing matched by an in-filing search.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentResponse {
    /// Document ID.
    pub document_id: Option<String>,
    /// Document title.
    pub title: Option<String>,
    /// Number of hits in the document.
    pub hits: Option<String>,
    /// Document URL.
    pub url: Option<String>,
    /// Document format.
    pub format: Option<String>,
    /// Excerpts (if highlighted).
    #[serde(default)]
    pub excerpts: Vec<ExcerptResponse>,
}

/// Response from `/global-filings/search-in-filing`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InFilingResponse {
    /// Filing ID.
    pub filing_id: Option<String>,
    /// Filing title.
    pub title: Option<String>,
    /// Filer entity ID.
    pub filer_id: Option<String>,
    /// Symbols associated with the filing.
    #[serde(default)]
    pub symbol: Option<serde_json::Value>,
    /// Filer name.
    pub name: Option<String>,
    /// Acceptance date.
    pub acceptance_date: Option<String>,
    /// Filed date.
    pub filed_date: Option<String>,
    /// Report date.
    pub report_date: Option<String>,
    /// Form type.
    pub form: Option<String>,
    /// Whether this is an amendment.
    pub amend: Option<bool>,
    /// Source identifier.
    pub source: Option<String>,
    /// Number of pages.
    pub page_count: Option<i64>,
    /// Number of documents.
    pub document_count: Option<i64>,
    /// Matching documents within the filing.
    #[serde(default)]
    pub documents: Vec<DocumentResponse>,
}
