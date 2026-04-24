# in-mem-threadsDB
A Simple In Memory `Arc<RwLock<HashMap<String, String>>>`, To Demo Rust `std::thread::*` Functionality.

# Core Aim
- To demo how threads, communication and atomic variables (`AtomicBool`) work in a simple OS-managed threading app.

# Idea
1. We spawn 6 threads that live as long as possible
2. Each thread acquired a `ReadLock` or `WriteLock` on a random data structure (in this case a `Hashmap<String, String>`
3. When a thread holds either lock, it sends a message to the main thread for what it's doing for it to print.
4. Multiple threads can hold a readlock at once - so we see logs by many threads doing a `GET` operation at the same time.
5. But only one thread a time can hold a writelock. When it does - all logs freeze until that writelock expires.

# Running
On a Linux terminal do:
> `RUST_LOG=debug cargo run`

# Sample Output
<img width="1588" height="793" alt="Screenshot_20260424_181813" src="https://github.com/user-attachments/assets/01de2fdd-66eb-4845-a30c-845bd5942b83" />

