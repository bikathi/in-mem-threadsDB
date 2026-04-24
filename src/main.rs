use std::sync::atomic::AtomicBool;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        mpsc::{self},
    },
    thread::{self, Builder},
    time::Duration,
};

use log::{error, info};

#[derive(Debug)]
enum Operation {
    GET,
    SET,
}

impl Operation {
    fn randomize_operation(min: i32, max: i32) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();

        // Use the modulo operator to stay within the range
        // This replicates: Math.floor(Math.random() * (max - min + 1)) + min
        let random = (nanos % (max - min + 1) as u128) as i32 + min;

        match random {
            1 => Self::GET,
            2 => Self::SET,
            _ => Self::randomize_operation(min, max),
        }
    }
}

fn main() {
    // init env_logger
    env_logger::init();

    // represents our in-mem DB
    let db: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));

    // the poison pill to kill the app
    let running = Arc::new(AtomicBool::new(true));

    // this is the messaging channel
    let (sender, receiver) = mpsc::channel::<String>();

    // initialize the Monitor threads
    for index in 1..=5 {
        let db_clone = Arc::clone(&db);
        let builder = Builder::new().name(format!("{}", index));
        let csender = sender.clone();
        let r = running.clone();
        match builder.spawn(move || {
            while r.load(std::sync::atomic::Ordering::SeqCst) {
                let operation = Operation::randomize_operation(1, 2);
                match operation {
                    // for read operations
                    Operation::GET => {
                        let _rl = db_clone.read().unwrap(); // create a readlock
                        csender
                            .send(format!(
                                "Thread {t} performing operation {o:?}",
                                t = index,
                                o = operation
                            ))
                            .unwrap();
                        // sleep with the lock for a while
                        thread::sleep(Duration::from_secs(2u64));
                    }
                    Operation::SET => {
                        let _wl = db_clone.write().unwrap(); // create a write lock
                        csender
                            .send(format!(
                                "Thread {t} performing operation {o:?}: (Sleeping)",
                                t = index,
                                o = operation
                            ))
                            .unwrap();
                        // sleep with the lock for a while
                        thread::sleep(Duration::from_secs(2u64));
                    }
                }

                csender
                    .send(format!(
                        "Thread {t} completed operation {o:?}",
                        t = index,
                        o = operation
                    ))
                    .unwrap();
            }
        }) {
            Ok(_) => (),
            Err(e) => {
                error!("Unable to create thread handle! Cause: {:?}", e);
                break;
            }
        }
    }

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        info!("Posion thread firing!");
        running.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    drop(sender);

    // the monitoring logic
    for received in receiver {
        info!("{received}. Active threads: {}", Arc::strong_count(&db));
    }

    handle.join().unwrap()
}
