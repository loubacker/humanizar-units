use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, OnRequest, OnResponse, TraceLayer,
};
use tracing::{Level, Span};

pub(crate) struct HttpObservability;

impl HttpObservability {
    pub(crate) fn apply(router: Router) -> Router {
        Self::apply_with_success_policy(router, true)
    }

    pub(crate) fn apply_healthcheck(router: Router) -> Router {
        Self::apply_with_success_policy(router, false)
    }

    fn apply_with_success_policy(router: Router, log_successful_responses: bool) -> Router {
        router.layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_request(move |request: &Request<Body>, span: &Span| {
                    if log_successful_responses {
                        DefaultOnRequest::new().on_request(request, span);
                    }
                })
                .on_response(
                    move |response: &Response<Body>, latency: Duration, span: &Span| {
                        if should_log(response.status(), log_successful_responses) {
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .on_response(response, latency, span);
                        }
                    },
                ),
        )
    }
}

fn should_log(status: StatusCode, log_successful_responses: bool) -> bool {
    log_successful_responses || !status.is_success()
}
