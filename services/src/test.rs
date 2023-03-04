use actix_web::{test, App};
use actix_web::http::{StatusCode};

use super::*;

#[actix_rt::test]
async fn test_index() {
    let mut app = test::init_service(
        App::new().route("/", web::get().to(index))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello, World!");
}

#[actix_rt::test]
async fn test_health() {
    let mut app = test::init_service(
        App::new().route("/health", web::get().to(health))
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(body, "200");
}

#[actix_rt::test]
async fn test_gold() {
    let mut app = test::init_service(
        App::new().route("/Gold", web::get().to(gold))
    ).await;

    let req = test::TestRequest::get().uri("/Gold").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Gold Status");
}

#[actix_rt::test]
async fn test_silver() {
    let mut app = test::init_service(
        App::new().route("/Silver", web::get().to(silver))
    ).await;

    let req = test::TestRequest::get().uri("/Silver").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Silver Status");
}

#[actix_rt::test]
async fn test_bronze() {
    let mut app = test::init_service(
        App::new().route("/Bronze", web::get().to(bronze))
    ).await;

    let req = test::TestRequest::get().uri("/Bronze").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Bronze Status");
}
