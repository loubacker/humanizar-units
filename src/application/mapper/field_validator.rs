use crate::domain::exception::UnitException;
use crate::domain::model::enums::ReasonCode;

pub fn required_field(field_name: &str, value: Option<&str>) -> Result<String, UnitException> {
    let Some(value) = value else {
        return Err(invalid_field(field_name));
    };

    if value.trim().is_empty() {
        return Err(invalid_field(field_name));
    }

    Ok(value.to_owned())
}

fn invalid_field(field_name: &str) -> UnitException {
    UnitException::with_message(
        ReasonCode::ValidationError,
        format!("Campo obrigatorio invalido: {field_name}."),
    )
}
