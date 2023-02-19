use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        body::boxed,
        response::IntoResponse,
        http::{StatusCode, Uri},
    };
    use axum::response::Response as AxumResponse;
    #[cfg(debug_assertions)]
    pub async fn file_and_error_handler(uri: Uri, axum::Extension(options): axum::Extension<std::sync::Arc<leptos::LeptosOptions>>, req: http::Request<axum::body::Body>) -> AxumResponse {
        use crate::error_template::*;
        use leptos::*;
        use tower::ServiceExt;
        use tower_http::services::ServeDir;
        use axum::body::Body;
        let options = &*options;
        let res = {
            let req = http::Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
            // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
            // This path is relative to the cargo root
            match ServeDir::new("target/site").oneshot(req).await {
                Ok(res) => Ok(res.map(boxed)),
                Err(err) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong: {err}"),
                )),
            }
        };
        match res {
            Ok(res) => res.into_response(),
            Err(_) => {
                let mut errors = Errors::default();
                errors.insert_with_default_key(AppError::NotFound);
                let handler = leptos_axum::render_app_to_stream(options.to_owned(), move |cx| view!{cx, <ErrorTemplate outside_errors=errors.clone()/>});
                handler(req).await.into_response()
            }
        }
    }
    #[cfg(not(debug_assertions))]
    pub async fn file_and_error_handler(uri: Uri) -> AxumResponse {
        use rust_embed::RustEmbed;
        use axum::body::Full;
        use http::header;
        #[derive(RustEmbed)]
        #[folder = "target/site/"]
        struct Asset;
        pub struct StaticFile<T>(pub T);

        impl<T> IntoResponse for StaticFile<T>
        where
          T: Into<String>,
        {
          fn into_response(self) -> AxumResponse {
            let path = self.0.into();

            match Asset::get(path.as_str()) {
              Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                AxumResponse::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
              }
              None => AxumResponse::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap(),
            }
          }
        }
        let path = uri.path().trim_start_matches('/').to_string();
        StaticFile(path).into_response()
    }
}}
