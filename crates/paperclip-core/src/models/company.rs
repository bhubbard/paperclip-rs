use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_company_status() -> String {
    "active".to_string()
}

fn default_issue_prefix() -> String {
    "PAP".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub mission: String,
    #[serde(default, alias = "description")]
    pub description: Option<String>,
    pub budget_limit_usd: f64,
    pub budget_spent_usd: f64,
    #[serde(default, rename = "budgetMonthlyCents")]
    pub budget_monthly_cents: Option<i64>,
    #[serde(default, rename = "spentMonthlyCents")]
    pub spent_monthly_cents: Option<i64>,
    pub currency: String,
    #[serde(default = "default_company_status")]
    pub status: String,
    #[serde(default = "default_issue_prefix", rename = "issuePrefix", alias = "issue_prefix")]
    pub issue_prefix: String,
    #[serde(default, rename = "issueCounter", alias = "issue_counter")]
    pub issue_counter: u32,
    #[serde(default, rename = "requireBoardApprovalForNewAgents", alias = "require_board_approval_for_new_agents")]
    pub require_board_approval_for_new_agents: bool,
    #[serde(rename = "createdAt", alias = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt", alias = "updated_at")]
    pub updated_at: DateTime<Utc>,
}

impl Company {
    pub fn new(name: impl Into<String>, mission: impl Into<String>, budget_limit_usd: f64) -> Self {
        let now = Utc::now();
        let name_str = name.into();
        let mission_str = mission.into();
        let prefix = derive_prefix(&name_str);
        Self {
            id: Uuid::new_v4().to_string(),
            name: name_str,
            mission: mission_str.clone(),
            description: Some(mission_str),
            budget_limit_usd,
            budget_spent_usd: 0.0,
            budget_monthly_cents: Some((budget_limit_usd * 100.0) as i64),
            spent_monthly_cents: Some(0),
            currency: "USD".to_string(),
            status: "active".to_string(),
            issue_prefix: prefix,
            issue_counter: 0,
            require_board_approval_for_new_agents: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn normalize_defaults(&mut self) {
        if self.description.is_none() {
            self.description = Some(self.mission.clone());
        }
        if self.budget_monthly_cents.is_none() {
            self.budget_monthly_cents = Some((self.budget_limit_usd * 100.0) as i64);
        }
        if self.spent_monthly_cents.is_none() {
            self.spent_monthly_cents = Some((self.budget_spent_usd * 100.0) as i64);
        }
        if self.issue_prefix.is_empty() {
            self.issue_prefix = derive_prefix(&self.name);
        }
        if self.status.is_empty() {
            self.status = "active".to_string();
        }
    }

    pub fn record_spend(&mut self, amount_usd: f64) {
        self.budget_spent_usd += amount_usd;
        self.spent_monthly_cents = Some((self.budget_spent_usd * 100.0) as i64);
        self.updated_at = Utc::now();
    }

    pub fn remaining_budget(&self) -> f64 {
        (self.budget_limit_usd - self.budget_spent_usd).max(0.0)
    }

    pub fn is_over_budget(&self) -> bool {
        self.budget_spent_usd >= self.budget_limit_usd
    }
}

fn derive_prefix(name: &str) -> String {
    let uppercase_chars: Vec<char> = name.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if uppercase_chars.is_empty() {
        return "PAP".to_string();
    }
    if uppercase_chars.len() <= 4 {
        uppercase_chars.into_iter().collect::<String>().to_uppercase()
    } else {
        uppercase_chars.into_iter().take(3).collect::<String>().to_uppercase()
    }
}
