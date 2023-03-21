use crate::protos::image_processing::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/// extract path info from "/image/{spec}/{url}" url
/// {spec} - spec of image processing of base64
/// {url} - the image url
#[get("/image/{spec}/{url}")] // <- define path parameters
async fn index(path: web::Path<(String, String)>) -> impl Responder {
    let (spec, url) = path.into_inner();

    // TODO
    let decoded_spec = ImageSpec::try_from(spec.as_str()).unwrap();
    let response_body = format!("the spec base64 is {}, and url is {}!", String::from(&decoded_spec), url);

    HttpResponse::Ok().body(response_body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_index() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get()
            .uri("/image/JAoKCggKBghkEGQgBAoOCgoSCAhkEGQYMiAyEAEKBgoCGgAQAg/example.png")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "the spec base64 is JAoKCggKBghkEGQgBAoOCgoSCAhkEGQYMiAyEAEKBgoCGgAQAg, and url is example.png!");
    }
}
