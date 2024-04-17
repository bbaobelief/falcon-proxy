use crate::config::AppConfig;
use axum::{
    body::Body,
    extract::{Json, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use reqwest::{
    header::{
        HeaderMap as ReqHeaderMap, HeaderName as ReqHeaderName, HeaderValue as ReqHeaderValue,
    },
    Client,
};
use serde_json::Value;

pub(crate) async fn get_config() -> Json<AppConfig> {
    Json(AppConfig::global().clone())
}

pub(crate) async fn get_health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}

pub(crate) async fn openfalcon_push(
    State(client): State<Client>,
    uri: Uri,
    header_map: HeaderMap,
    Json(payload): Json<Value>,
) -> Response {
    let mut headers = ReqHeaderMap::new();
    let cfg = AppConfig::global().clone();
    let url = match &cfg.falcon_agent_addr {
        Some(addr) => addr.clone(),
        None => {
            if uri.path().starts_with("/v1/push") {
                format!("{}{}", cfg.n9e_server.trim(), "/openfalcon/push")
            } else {
                format!("{}{}", cfg.n9e_server.trim(), uri)
            }
        }
    };

    // Set request header
    for key in cfg.allow_headers.iter() {
        let name = key.to_lowercase();
        let header_name = ReqHeaderName::from_bytes(name.as_ref()).unwrap();
        if let Some(value) = header_map.get(&name) {
            let header_value = ReqHeaderValue::from_bytes(value.as_bytes()).unwrap();
            headers.insert(header_name, header_value);
        }
    }
    if let Some(abbr) = &cfg.monitor_company_abbr {
        let header_value = ReqHeaderValue::from_bytes(abbr.as_bytes()).unwrap();
        headers.insert("monitor-company-abbr", header_value);
    }

    tracing::debug!("url: {}", url);
    tracing::debug!("headers: {:?}", headers);
    tracing::debug!("payload: {}", payload.to_string());

    let response = match client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            tracing::error!(%err, "request failed");
            return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
        }
    };

    // Here the mapping of headers is required due to reqwest and axum differ on the http crate versions
    let mut response_builder = Response::builder().status(response.status().as_u16());
    let headers = response.headers();
    for (name, value) in headers {
        let name = HeaderName::from_bytes(name.as_ref()).unwrap();
        let value = if name == "server" {
            HeaderValue::from_static("axum")
        } else {
            HeaderValue::from_bytes(value.as_bytes()).unwrap()
        };
        response_builder = response_builder.header(name, value);
    }
    response_builder
        .body(Body::from_stream(response.bytes_stream()))
        .unwrap()
}
