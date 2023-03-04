use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;

use futures::{future, FutureExt};
use hyper::service::make_service_fn;
use warp::Filter;

fn main() {
    let svc = warp::service(
        warp::path::end()
            .map(|| "Hello, world")
            .with(warp::trace::request()),
    );
    let make_svc = make_service_fn(move |_| future::ok::<_, Infallible>(svc.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    tokio::spawn(hyper::Server::bind(&addr).serve(make_svc));
}
