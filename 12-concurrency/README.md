# 12. Concurrency

## Native Threads vs. Green Threads

Native Threads - managed by the OS

Green Threads - multiple green threads map to a native thread.

> [!IMPORTANT]
> OOTB Rust supports only Native Threads.

> [!IMPORTANT]
> In Rust many concurrency errors are caught at compile-time.
