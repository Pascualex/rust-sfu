# SFU

To run:

    cargo run --release

To configure number of worker threads edit the following:

    #[tokio::main(flavor = "current_thread")]
    // #[tokio::main(flavor = "multi_thread", worker_threads = 8)]
    async fn main() {
        // ...
    }
