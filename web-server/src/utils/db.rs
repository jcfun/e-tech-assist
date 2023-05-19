use crate::config::init::get_ctx;
use rbatis::executor::RBatisTxExecutorGuard;

pub async fn get_tx() -> RBatisTxExecutorGuard {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    })
}
