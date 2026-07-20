use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    subject: String,
    client_id: Option<String>,
    roles: HashSet<String>,
}

impl AuthenticatedUser {
    pub(crate) fn new(subject: String, client_id: Option<String>, roles: HashSet<String>) -> Self {
        Self {
            subject,
            client_id,
            roles,
        }
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn client_id(&self) -> Option<&str> {
        self.client_id.as_deref()
    }

    pub fn roles(&self) -> &HashSet<String> {
        &self.roles
    }

    pub fn has_role(&self, required_role: &str) -> bool {
        let normalized = normalize_role(required_role);
        !normalized.is_empty() && self.roles.contains(&normalized)
    }
}

pub(crate) fn normalize_role(role: &str) -> String {
    role.trim()
        .strip_prefix("ROLE_")
        .unwrap_or(role.trim())
        .trim()
        .to_ascii_uppercase()
}
