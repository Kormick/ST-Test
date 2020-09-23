use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Result};

use std::sync::{Arc, RwLock};

use crate::assignment::{Assignment, InputSet};

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

pub async fn run_actix_app() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let data = web::Data::new(Arc::new(RwLock::new(
            Assignment::new().with_rules(true, true),
        )));

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
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
    use actix_web::{test, web, App};

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
}
