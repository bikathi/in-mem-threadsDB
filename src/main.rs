use std::{
    collections::HashMap,
    sync::{Arc, RwLock, mpsc},
};

use log::info;

fn main() {
    // init env_logger
    env_logger::init();

    // this is the key data structure we will be editting over threads
    let db: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));

    // this is the messaging channel
    let (sender, receiver) = mpsc::channel::<&str>();

    // the monitoring logic
    for received in receiver.iter() {
        info!("{received}")
    }
}
