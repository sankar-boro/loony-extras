use loony_identity::{IdentityService, CookieIdentityPolicy, Identity};
use loony::http::{StatusCode, header};
use loony::web::test::{self, TestRequest};
use loony::web::{self, App, HttpResponse};

const COOKIE_KEY_MASTER: [u8; 32] = [0; 32];
const COOKIE_NAME: &'static str = "loony_auth";
const COOKIE_LOGIN: &'static str = "test";

#[loony::test]
async fn test_identity() {
    let mut srv = test::init_service(
    App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&COOKIE_KEY_MASTER)
                .domain("www.rust-lang.org")
                .name(COOKIE_NAME)
                .path("/")
                .secure(true)
            ))
            .service(web::resource("/index").to(|id: Identity| async move {
                if id.identity().is_some() {
                    HttpResponse::Created()
                } else {
                    HttpResponse::Ok()
                }
            }))
            .service(web::resource("/login").to(|id: Identity| async move {
                id.remember(COOKIE_LOGIN.to_string());
                HttpResponse::Ok()
            }))
            .service(web::resource("/logout").to(|id: Identity| async move {
                if id.identity().is_some() {
                    id.forget();
                    HttpResponse::Ok()
                } else {
                    HttpResponse::BadRequest()
                }
            })),
        )
        .await;
    let resp =
        test::call_service(&mut srv, TestRequest::with_uri("/index").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp =
        test::call_service(&mut srv, TestRequest::with_uri("/login").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let c = resp.response().cookies().next().unwrap().to_owned();

    let resp = test::call_service(
        &mut srv,
        TestRequest::with_uri("/index").cookie(c.clone()).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let resp = test::call_service(
        &mut srv,
        TestRequest::with_uri("/logout").cookie(c.clone()).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(resp.headers().contains_key(header::SET_COOKIE))
}