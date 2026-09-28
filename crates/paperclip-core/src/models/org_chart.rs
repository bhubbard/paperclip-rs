use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::error::PaperclipError;
use super::agent::Agent;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OrgNode {
    pub agent_id: String,
    pub name: String,
    pub role: String,
    pub budget_limit_usd: f64,
    pub budget_spent_usd: f64,
    pub children: Vec<OrgNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OrgChart {
    pub roots: Vec<OrgNode>,
}

impl OrgChart {
    pub fn build(agents: &[Agent]) -> Result<Self, PaperclipError> {
        let mut by_id: HashMap<String, &Agent> = HashMap::new();
        let mut children_map: HashMap<Option<String>, Vec<String>> = HashMap::new();

        for agent in agents {
            by_id.insert(agent.id.clone(), agent);
            children_map
                .entry(agent.reports_to.clone())
                .or_default()
                .push(agent.id.clone());
        }

        // Cycle detection
        for agent in agents {
            let mut visited = HashSet::new();
            let mut curr = Some(agent.id.clone());
            while let Some(id) = curr {
                if !visited.insert(id.clone()) {
                    return Err(PaperclipError::InvalidOrgChart(format!(
                        "Cyclic reporting structure detected at agent: {}",
                        id
                    )));
                }
                curr = by_id.get(&id).and_then(|a| a.reports_to.clone());
            }
        }

        fn build_node(agent_id: &str, by_id: &HashMap<String, &Agent>, children_map: &HashMap<Option<String>, Vec<String>>) -> OrgNode {
            let agent = by_id.get(agent_id).expect("Agent should exist");
            let child_ids = children_map.get(&Some(agent_id.to_string())).cloned().unwrap_or_default();
            let children = child_ids
                .into_iter()
                .map(|cid| build_node(&cid, by_id, children_map))
                .collect();

            OrgNode {
                agent_id: agent.id.clone(),
                name: agent.name.clone(),
                role: agent.role.to_string(),
                budget_limit_usd: agent.budget_limit_usd,
                budget_spent_usd: agent.budget_spent_usd,
                children,
            }
        }

        let root_ids = children_map.get(&None).cloned().unwrap_or_default();
        let roots = root_ids
            .into_iter()
            .map(|rid| build_node(&rid, &by_id, &children_map))
            .collect();

        Ok(Self { roots })
    }

    pub fn render_ascii(&self) -> String {
        let mut out = String::new();
        for (i, root) in self.roots.iter().enumerate() {
            Self::render_node(root, "", true, &mut out);
            if i + 1 < self.roots.len() {
                out.push('\n');
            }
        }
        out
    }

    fn render_node(node: &OrgNode, prefix: &str, is_last: bool, out: &mut String) {
        let branch = if prefix.is_empty() {
            "◆ "
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

        out.push_str(&format!(
            "{}{}{} ({}) [${:.2} / ${:.2}]\n",
            prefix, branch, node.name, node.role, node.budget_spent_usd, node.budget_limit_usd
        ));

        let child_prefix = if prefix.is_empty() {
            "  "
        } else if is_last {
            &format!("{}    ", prefix)
        } else {
            &format!("{}│   ", prefix)
        };

        let count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            Self::render_node(child, child_prefix, i + 1 == count, out);
        }
    }
}
