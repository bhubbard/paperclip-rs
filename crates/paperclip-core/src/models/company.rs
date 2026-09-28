use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub mission: String,
    pub budget_limit_usd: f64,
    pub budget_spent_usd: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Company {
    pub fn new(name: impl Into<String>, mission: impl Into<String>, budget_limit_usd: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            mission: mission.into(),
            budget_limit_usd,
            budget_spent_usd: 0.0,
            currency: "USD".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn record_spend(&mut self, amount_usd: f64) {
        self.budget_spent_usd += amount_usd;
        self.updated_at = Utc::now();
    }

    pub fn remaining_budget(&self) -> f64 {
        (self.budget_limit_usd - self.budget_spent_usd).max(0.0)
    }

    pub fn is_over_budget(&self) -> bool {
        self.budget_spent_usd >= self.budget_limit_usd
    }
}
