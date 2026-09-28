use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaperclipError {
    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid org chart: {0}")]
    InvalidOrgChart(String),

    #[error("Budget limit exceeded: {0}")]
    BudgetExceeded(String),

    #[error("Approval required for action: {0}")]
    ApprovalRequired(String),

    #[error("Agent execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
