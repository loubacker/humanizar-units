use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HealthResponseDto {
    status: &'static str,
}

impl HealthResponseDto {
    pub const fn up() -> Self {
        Self { status: "UP" }
    }

    pub const fn status(&self) -> &str {
        self.status
    }
}
