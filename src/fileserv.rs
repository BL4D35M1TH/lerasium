use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        body::boxed,
        response::IntoResponse,
        http::{StatusCode, Uri},
    };
    use axum::response::Response as AxumResponse;
    pub async fn file_and_error_handler(uri: Uri, axum::Extension(options): axum::Extension<std::sync::Arc<leptos::LeptosOptions>>, req: http::Request<axum::body::Body>) -> AxumResponse {
        use crate::error_template::*;
        use leptos::*;
        use tower::ServiceExt;
        use tower_http::services::ServeDir;
        use axum::body::Body;
        let options = &*options;
        let root = options.site_root.clone();
        let res = {
            let req = http::Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
            // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
            // This path is relative to the cargo root
            match ServeDir::new(&root).oneshot(req).await {
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
}}
