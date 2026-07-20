use std::collections::HashSet;

use serde::Deserialize;

use super::authenticated_user::{AuthenticatedUser, normalize_role};

#[derive(Debug, Deserialize)]
pub(crate) struct JwtClaims {
    sub: String,
    #[serde(default)]
    client_id: Option<String>,
    #[serde(default)]
    role: Option<RoleClaim>,
    #[serde(default)]
    roles: Option<RoleClaim>,
}

impl JwtClaims {
    pub(crate) fn subject_is_empty(&self) -> bool {
        self.sub.trim().is_empty()
    }

    pub(crate) fn into_authenticated_user(self) -> AuthenticatedUser {
        let mut roles = HashSet::new();

        append_roles(&mut roles, self.role);
        append_roles(&mut roles, self.roles);

        AuthenticatedUser::new(self.sub, self.client_id, roles)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RoleClaim {
    Text(String),
    List(Vec<String>),
}

fn append_roles(roles: &mut HashSet<String>, claim: Option<RoleClaim>) {
    let Some(claim) = claim else {
        return;
    };

    match claim {
        RoleClaim::Text(value) => {
            for role in value.split(',') {
                insert_role(roles, role);
            }
        }
        RoleClaim::List(values) => {
            for value in values {
                for role in value.split(',') {
                    insert_role(roles, role);
                }
            }
        }
    }
}

fn insert_role(roles: &mut HashSet<String>, role: &str) {
    let role = normalize_role(role);

    if !role.is_empty() {
        roles.insert(role);
    }
}
