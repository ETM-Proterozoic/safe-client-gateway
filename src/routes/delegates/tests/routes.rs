use core::time::Duration;
use mockall::predicate::eq;
use crate::common::models::page::Page;
use crate::routes::delegates::models::{Delegate, DelegateCreate, DelegateDelete, SafeDelegateDelete};
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use crate::tests::main::setup_rocket;
use crate::utils::http_client::{MockHttpClient, Request, Response};

use crate::config::{default_request_timeout, chain_info_request_timeout};

#[rocket::async_test]
async fn get_delegates_from_safe() {
    let safe_address = "0xaE3c91c89153DEaC332Ab7BBd167164978638c30";
    // Mocking response of rinkeby chain
    let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    let mut mock_http_client = MockHttpClient::new();
    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });
    //Mocking response of transaction service delegates
    let mut delegates_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/delegates/?safe={}&delegate=&delegator=&label=",
        &safe_address));
    delegates_request.timeout(Duration::from_millis(default_request_timeout()));

   
    mock_http_client
    .expect_get()
    .times(1)
    .with(eq(delegates_request))
    .returning(move |_| {
        Ok(Response {
            body: String::from(super::BACKEND_LIST_DELEGATES_OF_SAFE),
            status_code: 200,
        })
    });
    
    // setup route with mocked data
    let client = Client::tracked(
        setup_rocket(mock_http_client, routes![super::super::routes::get_delegates]).await,
    )
    .await
    .expect("valid rocket instance");

    let expected =
        super::EXPECTED_LIST_DELEGATES_OF_SAFE;
    
    // Requesting delegates to client-gateway
    let request = client
        .get(format!("/v1/chains/{}/delegates?safe={}", 4, &safe_address))
        .header(Header::new("Host", "test.gnosis.io/api"))
        .header(ContentType::JSON);
    
    let response = request.dispatch().await; 
    let actual_status = response.status();
    let actual_json_body = response.into_string().await.unwrap();

    
    assert_eq!(actual_status, Status::Ok);
    //assert_eq!(actual_json_body, expected);
}