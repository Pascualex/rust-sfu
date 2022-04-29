# SFU Load

Load test used to characterize performance.

To run:

    cargo run --release

To configure the test edit the following parameters:

    async fn main() {
        let clients = 150;
        let max_subs = 49;
        let duration = Duration::from_secs(5);
        let size = 30_000;
        let freq = 24;

        // ...
    }
