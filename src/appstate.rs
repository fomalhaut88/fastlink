use std::sync::Mutex;

use crate::DBConnector;


pub struct AppState {
    pub db: Mutex<DBConnector>,
    pub order: usize,
    pub prime: u64,
    pub block_size: usize,
}
