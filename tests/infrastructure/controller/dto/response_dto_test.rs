#![forbid(unsafe_code)]

use chrono::{TimeZone, Utc};
use humanizar_units::infrastructure::controller::dto::{ErrorResponseDto, SuccessResponseDto};
use serde_json::json;

#[test]
fn should_serialize_shared_success_contract_in_camel_case() {
    let timestamp = Utc.with_ymd_and_hms(2026, 7, 17, 12, 0, 0).unwrap();
    let response = SuccessResponseDto::with_timestamp(
        "UNIT_CREATED",
        "4d748765-8729-4a7f-baa9-2681a568863c",
        "Unidade criada com sucesso.",
        json!({ "unitId": "8da67fcb-1539-411f-bf9d-61430d43cc63" }),
        timestamp,
    );

    let serialized = serde_json::to_value(response).unwrap();

    assert_eq!(
        serialized,
        json!({
            "code": "UNIT_CREATED",
            "success": true,
            "correlationId": "4d748765-8729-4a7f-baa9-2681a568863c",
            "message": "Unidade criada com sucesso.",
            "data": {
                "unitId": "8da67fcb-1539-411f-bf9d-61430d43cc63"
            },
            "timestamp": "2026-07-17T12:00:00Z"
        })
    );
}

#[test]
fn should_serialize_shared_error_contract_in_camel_case() {
    let timestamp = Utc.with_ymd_and_hms(2026, 7, 17, 12, 0, 0).unwrap();
    let response = ErrorResponseDto::with_timestamp(
        timestamp,
        404,
        "Not Found",
        "UNIT_NOT_FOUND",
        "Unidade não encontrada.",
        "/api/v1/municipio/123/units/456",
    );

    let serialized = serde_json::to_value(response).unwrap();

    assert_eq!(
        serialized,
        json!({
            "timestamp": "2026-07-17T12:00:00Z",
            "status": 404,
            "error": "Not Found",
            "reasonCode": "UNIT_NOT_FOUND",
            "message": "Unidade não encontrada.",
            "path": "/api/v1/municipio/123/units/456"
        })
    );
}
