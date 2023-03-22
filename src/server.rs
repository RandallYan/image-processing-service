use crate::protos::image_processing::*;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use self::cache::AppState;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    let cache = LruCache::new(NonZeroUsize::new(10).unwrap());
    let cache = Arc::new(Mutex::new(cache));
    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                cache: cache.clone(),
            })
            .service(index)
            .route("/pro", web::get().to(image))
    })
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
    let response_body = format!(
        "the spec base64 is {}, and url is {}!",
        String::from(&decoded_spec),
        url
    );

    HttpResponse::Ok().body(response_body)
}

async fn image(data: web::Data<cache::AppState>, req: HttpRequest) -> impl Responder {
    let _spec = req.match_info().get("spec").unwrap().to_owned();
    let url = req.match_info().get("key").unwrap().to_owned();

    let value = {
        let mut cache = data.cache.lock().unwrap();
        if let Some(value) = cache.get(&url) {
            value.to_owned()
        } else {
            let value = format!("value for key: {}", &url);
            cache.put(url, value.clone());
            value
        }
    };
    // TODO
    // let decoded_spec = ImageSpec::try_from(spec.as_str()).unwrap();
    // let response_body = format!("the spec base64 is {}, and url is {}!", String::from(&decoded_spec), String::from(url.ToString()));
    HttpResponse::Ok().body(value)
}

pub mod cache {
    use lru::LruCache;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct AppState {
        pub cache: Arc<Mutex<LruCache<String, String>>>,
    }

    use crate::protos::image_processing::ImageSpec;

    pub struct Cache {
        pub cache: Arc<Mutex<HashMap<String, ImageSpec>>>,
    }

    impl Cache {
        pub fn new() -> Self {
            Cache {
                cache: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }
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
