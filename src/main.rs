use ::http::StatusCode;
use ::http::Uri;
use axum::extract::Host;
use axum::handler::HandlerWithoutStateExt;
use axum::response::Redirect;
use axum::BoxError;
use axum_server::tls_rustls::RustlsConfig;
use http::routes::app_routes;
use rustls::crypto::ring::default_provider;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;

mod config;
mod http;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("Server running on port {}", listener.local_addr().unwrap());
    axum::serve(listener, app_routes()).await.unwrap();

    // default_provider()
    //     .install_default()
    //     .expect("failed to install default provider");
    // let ports = Ports {
    //     http: 8080,
    //     https: 2443,
    // };

    // // tokio::spawn(redirect_http_to_https(ports));

    // let config = RustlsConfig::from_pem_file(
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("files")
    //         .join("cert.pem"),
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("files")
    //         .join("key.pem"),
    // )
    // .await
    // .unwrap();

    // let addr = SocketAddr::from(([0, 0, 0, 0], ports.https));
    // tracing::debug!("listening on {}", addr);
    // println!("Server running on port {}", addr);
    // axum_server::bind_rustls(addr, config)
    //     .serve(app_routes().into_make_service())
    //     .await
    //     .unwrap();
}

#[allow(dead_code)]
async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                println!("failed to convert URI to HTTPS {}", error);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("Server running on port {}", listener.local_addr().unwrap());
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}
