use std::error::Error;

const FAILURE_PREFIX: &str = "Falha ao iniciar humanizar-units";
const CAUSE_LIMIT: usize = 16;

pub struct StartupReport;

impl StartupReport {
    pub fn failure(error: &(dyn Error + 'static)) -> String {
        let mut report = format!("{FAILURE_PREFIX}: {error}");

        for (position, cause) in causes(error).enumerate() {
            report.push_str(&format!("\n  causa {}: {cause}", position + 1));
        }

        report
    }
}

fn causes<'a>(error: &'a (dyn Error + 'static)) -> impl Iterator<Item = &'a (dyn Error + 'static)> {
    let mut current = error.source();

    std::iter::from_fn(move || {
        let cause = current?;
        current = cause.source();

        Some(cause)
    })
    .take(CAUSE_LIMIT)
}
