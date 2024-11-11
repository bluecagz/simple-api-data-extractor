use simple_api_data_extractor::{coinmarketcap::routes, config::load_config};
use actix_web::{test, App};

#[actix_web::test]
async fn test_get_latest() {
    let config = load_config().expect("Failed to load config");
    let app = test::init_service(App::new().app_data(config).service(routes::get_latest)).await;

    let req = test::TestRequest::get().uri("/latest").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_historical() {
    let config = load_config().expect("Failed to load config");
    let app = test::init_service(App::new().app_data(config).service(routes::get_historical)).await;

    let req = test::TestRequest::get().uri("/historical").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
