pub trait AsyncBridgeAggregate: Send + Sync {}

pub async fn run_async<F, T>(f: F) -> T where F: std::future::Future<Output = T> {
    f.await
}
