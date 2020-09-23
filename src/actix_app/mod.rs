//! Implements simple actix application for assignment.

use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

use std::sync::{Arc, RwLock};

use crate::assignment::{arithmetic_rule::SubstitutionToken, Assignment, InputSet};

#[derive(Serialize, Deserialize)]
pub struct AddRuleReq {
    token: SubstitutionToken,
    rule_str: String,
}

/// Endpoint to add new `LogicalRule` to `Assignment`.
/// Accepts `AddRuleReq` in JSON format.
///
/// Returns `HttpResponse::Ok()` if new rule added successfully,
/// otherwise returns `HttpResponse::BadRequest` with error message in JSON.
#[post("/add_logical_rule")]
pub async fn add_logical_rule(
    data: web::Data<Arc<RwLock<Assignment>>>,
    item: web::Json<AddRuleReq>,
) -> Result<HttpResponse> {
    let mut data = (*data).write().unwrap();
    let res = data.add_logical_rule_from_str(item.token.clone(), item.rule_str.clone());

    if res.is_err() {
        Ok(HttpResponse::BadRequest().json(res.unwrap_err().to_string()))
    } else {
        Ok(HttpResponse::Ok().json(res.unwrap()))
    }
}

/// Endpoint to add new `ArithmeticRule` to `Assignment`.
/// Accepts `AddRuleReq` in JSON format.
///
/// Returns `HttpResponse::Ok()` if new rule added successfully,
/// otherwise returns `HttpResponse::BadRequest` with error message in JSON.
#[post("/add_arithmetic_rule")]
pub async fn add_arithmetic_rule(
    data: web::Data<Arc<RwLock<Assignment>>>,
    item: web::Json<AddRuleReq>,
) -> Result<HttpResponse> {
    let mut data = (*data).write().unwrap();
    let res = data.add_arithmetic_rule_from_str(item.token.clone(), item.rule_str.clone());

    if res.is_err() {
        Ok(HttpResponse::BadRequest().json(res.unwrap_err().to_string()))
    } else {
        Ok(HttpResponse::Ok().json(res.unwrap()))
    }
}

/// Endpoint for assignment calculation.
/// Accepts `InputSet` in JSON format.
///
/// If calculation is successful, returns `HttpResponse::Ok()` with result in JSON,
/// otherwise `HttpResponse::BadRequest()` with error message in JSON.
#[post("/eval")]
pub async fn eval(
    data: web::Data<Arc<RwLock<Assignment>>>,
    item: web::Json<InputSet>,
) -> Result<HttpResponse> {
    let data = (*data).read().unwrap();
    let res = data.eval(item.0);

    if res.is_err() {
        Ok(HttpResponse::BadRequest().json(res.unwrap_err().to_string()))
    } else {
        Ok(HttpResponse::Ok().json(res.unwrap()))
    }
}

/// Creates and runs `HttpServer`, adds `Assignment` as server application data and binds endpoints.
pub async fn run_actix_app() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let data = web::Data::new(Arc::new(RwLock::new(
        Assignment::new().with_rules(true, true),
    )));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .service(add_logical_rule)
            .service(add_arithmetic_rule)
            .service(eval)
    })
    .bind("127.0.0.25:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assignment::arithmetic_rule::SubstitutionToken;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_add_logical_rule() {
        let data = web::Data::new(Arc::new(RwLock::new(Assignment::new())));
        let mut app =
            test::init_service(App::new().app_data(data.clone()).service(add_logical_rule)).await;

        let req = test::TestRequest::post()
            .uri("/add_logical_rule")
            .set_json(&AddRuleReq {
                token: SubstitutionToken::M,
                rule_str: "A && B".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let req = test::TestRequest::post()
            .uri("/add_logical_rule")
            .set_json(&AddRuleReq {
                token: SubstitutionToken::M,
                rule_str: "A + B".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_add_arithmetic_rule() {
        let data = web::Data::new(Arc::new(RwLock::new(Assignment::new())));
        let mut app = test::init_service(
            App::new()
                .app_data(data.clone())
                .service(add_arithmetic_rule),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/add_arithmetic_rule")
            .set_json(&AddRuleReq {
                token: SubstitutionToken::M,
                rule_str: "D + E".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let req = test::TestRequest::post()
            .uri("/add_arithmetic_rule")
            .set_json(&AddRuleReq {
                token: SubstitutionToken::M,
                rule_str: "D && E".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_eval_empty() {
        let data = web::Data::new(Arc::new(RwLock::new(Assignment::new())));
        let mut app = test::init_service(App::new().app_data(data.clone()).service(eval)).await;

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet::default())
            .to_request();
        let resp: String = test::read_response_json(&mut app, req).await;
        assert_eq!(resp, "Failed to apply logical rule.");
    }

    #[actix_rt::test]
    async fn test_eval_base_rules() {
        let data = web::Data::new(Arc::new(RwLock::new(
            Assignment::new().with_rules(true, false),
        )));
        let mut app = test::init_service(App::new().app_data(data.clone()).service(eval)).await;

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet::default())
            .to_request();
        let resp: String = test::read_response_json(&mut app, req).await;
        assert_eq!(resp, "Failed to apply logical rule.");

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet {
                a: true,
                b: true,
                c: false,
                d: 2.0,
                e: 3,
                f: 4,
            })
            .to_request();
        let resp: (SubstitutionToken, f64) = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.0, SubstitutionToken::M);
        assert_eq!(resp.1, 2.6);
    }

    #[actix_rt::test]
    async fn test_eval_override_rules() {
        let data = web::Data::new(Arc::new(RwLock::new(
            Assignment::new().with_rules(true, true),
        )));
        let mut app = test::init_service(App::new().app_data(data.clone()).service(eval)).await;

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet::default())
            .to_request();
        let resp: String = test::read_response_json(&mut app, req).await;
        assert_eq!(resp, "Failed to apply logical rule.");

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet {
                a: true,
                b: true,
                c: false,
                d: 2.0,
                e: 3,
                f: 15,
            })
            .to_request();
        let resp: (SubstitutionToken, f64) = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.0, SubstitutionToken::T);
        assert_eq!(resp.1, 1.0);
    }

    #[actix_rt::test]
    async fn test_eval_str_rules() {
        let data = web::Data::new(Arc::new(RwLock::new(Assignment::new())));
        let mut app = test::init_service(
            App::new()
                .app_data(data.clone())
                .service(add_logical_rule)
                .service(add_arithmetic_rule)
                .service(eval),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/add_logical_rule")
            .set_json(&AddRuleReq {
                token: SubstitutionToken::M,
                rule_str: "A && B".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let req = test::TestRequest::post()
            .uri("/add_arithmetic_rule")
            .set_json(&AddRuleReq {
                token: SubstitutionToken::M,
                rule_str: "D + E".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet::default())
            .to_request();
        let resp: String = test::read_response_json(&mut app, req).await;
        assert_eq!(resp, "Failed to apply logical rule.");

        let req = test::TestRequest::post()
            .uri("/eval")
            .set_json(&InputSet {
                a: true,
                b: true,
                c: false,
                d: 1.0,
                e: 2,
                f: 0,
            })
            .to_request();
        let resp: (SubstitutionToken, f64) = test::read_response_json(&mut app, req).await;
        assert_eq!(resp, (SubstitutionToken::M, 3.0));
    }
}
