use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("Provider execution error: {0}")]
    Provider(String),

    #[error("Execution timed out after {0} seconds")]
    Timeout(u64),

    #[error("Agent {0} is currently paused or terminated")]
    AgentNotActive(String),

    #[error("Budget limit reached: {0}")]
    BudgetHalted(String),

    #[error("Storage error: {0}")]
    Storage(#[from] paperclip_core::error::PaperclipError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
