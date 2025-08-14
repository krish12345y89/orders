use heed::types::SerdeBincode;

use crate::schema::{ order::Order };
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DB {
    pub env: heed::Env,
    pub order_db: heed::Database<SerdeBincode<String>, SerdeBincode<Order>>,
}

pub async fn init_db<P: AsRef<std::path::Path>>(path: P) -> Result<DB, anyhow::Error> {
    let env = unsafe {
        heed::EnvOpenOptions
            ::new()
            .map_size(1024 * 1024 * 1024) // 1GB
            .max_dbs(3)
            .open(path)?
    };
    let new_env = env.clone();
    let mut txn = new_env.write_txn()?;
    let order_db = env
        .create_database(&mut txn, Some("orders"))
        .expect("Failed to create orders database");
    txn.commit()?;

    Ok(DB {
        env,
        order_db,
    })
}
