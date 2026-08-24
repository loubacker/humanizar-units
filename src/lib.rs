#![forbid(unsafe_code)]

// ===== Domain =====

pub mod domain {
    pub mod exception {
        mod persistence_exception;
        mod reason_code_exception;
        mod security_exception;
        mod technical_error;
        mod unit_exception;

        pub use persistence_exception::PersistenceException;
        pub use reason_code_exception::ReasonCodeException;
        pub use security_exception::SecurityException;
        pub use technical_error::TechnicalError;
        pub use unit_exception::UnitException;
    }

    pub mod model {
        pub mod enums {
            mod reason_code;

            pub use reason_code::ReasonCode;
        }

        mod municipio;
        mod unit;

        pub use municipio::Municipio;
        pub use unit::Unit;
    }

    pub mod port {
        mod municipio_port;
        mod unit_port;

        pub use municipio_port::MunicipioPort;
        pub use unit_port::UnitPort;
    }
}

// ===== Application =====

pub mod application {
    pub mod dto {
        mod municipio_dto;
        mod unit_dto;
        mod unit_ids_query_dto;

        pub use municipio_dto::MunicipioDto;
        pub use unit_dto::UnitDto;
        pub use unit_ids_query_dto::UnitIdsQueryDto;
    }

    pub mod mapper {
        mod field_validator;
        mod municipio_mapper;
        mod unit_mapper;

        pub use municipio_mapper::MunicipioMapper;
        pub use unit_mapper::UnitMapper;
    }

    pub mod service {
        mod municipio_service;
        mod unit_service;

        pub use municipio_service::MunicipioService;
        pub use unit_service::UnitService;
    }

    pub mod usecase {
        mod lookup;
        pub mod municipio_usecase;
        pub mod unit_usecase;
    }

    pub use dto::{MunicipioDto, UnitDto, UnitIdsQueryDto};
    pub use service::{MunicipioService, UnitService};
}

// ===== Infrastructure =====

pub mod infrastructure {
    pub mod adapter {
        mod municipio_adapter;
        mod unit_adapter;

        pub use municipio_adapter::MunicipioAdapter;
        pub use unit_adapter::UnitAdapter;
    }

    pub mod config {
        mod cors_config;
        mod database_config;
        mod environment_config;
        mod retry_config;
        mod security_config;
        mod server_config;

        pub use cors_config::{ApplicationEnvironment, CorsConfig, CorsConfigError};
        pub use database_config::{DatabaseConfig, DatabaseConfigError, DatabaseSettings};
        pub use environment_config::{EnvironmentConfig, EnvironmentConfigError};
        pub use retry_config::{RetryConfig, RetryConfigError, RetrySettings};
        pub use security_config::{SecurityConfig, SecurityConfigError, SecuritySettings};
        pub use server_config::{ServerConfig, ServerConfigError};
    }

    pub mod controller {
        pub mod dto {
            mod error_response_dto;
            mod health_response_dto;
            mod success_response_dto;

            pub use error_response_dto::ErrorResponseDto;
            pub use health_response_dto::HealthResponseDto;
            pub use success_response_dto::SuccessResponseDto;
        }

        mod extractor;

        pub mod handler {
            mod http_error;

            pub use http_error::{
                HttpError, PersistenceHttpError, SecurityHttpError, UnitHttpError,
            };
        }

        mod health_controller;
        mod municipio_controller;
        mod result_ext;
        mod router;
        mod state;
        mod unit_controller;

        pub use router::create_router;
        pub use state::ApplicationState;
    }

    pub mod diagnostics {
        mod safe_url;
        mod startup_report;

        pub use safe_url::SafeUrl;
        pub use startup_report::StartupReport;
    }

    pub mod handler {
        mod postgres_error_handler;

        pub use postgres_error_handler::PostgresErrorHandler;
    }

    pub mod persistence {
        pub mod entity {
            mod municipio_entity;
            mod unit_entity;

            pub use municipio_entity::MunicipioEntity;
            pub use unit_entity::UnitEntity;
        }

        pub mod repository {
            mod municipio_repository;
            mod unit_repository;

            pub use municipio_repository::MunicipioRepository;
            pub use unit_repository::UnitRepository;
        }

        pub use repository::{MunicipioRepository, UnitRepository};
    }

    pub mod resilience {
        mod retry_executor;

        pub use retry_executor::RetryExecutor;
    }

    pub mod security {
        mod authenticated_user;
        mod authorization;
        mod jwks_cache;
        mod jwt_claims;
        mod jwt_validator;

        pub use authenticated_user::AuthenticatedUser;
        pub(crate) use authorization::{authenticate, require_administrator};
        pub(crate) use jwks_cache::{JwksCache, JwksHttpTimeouts};
        pub(crate) use jwt_validator::JwtValidator;
    }

    pub mod server;
}
