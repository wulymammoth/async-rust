# ch01 - getting started : 1.1 why async?

- async is about concurrency -- performing multiple tasks concurrently on the same OS thread
- lots of overhead involved in the process of switching between different threads and sharing data between threads
  - threads that sit and do nothing still use up valuable system resources
- async is supposed to aid in reducing the footprint of the above
- `futures::join` is like a wait group for async functions
  ```rust
  async fn async_main() {
      let f1 = learn_and_sing();
      let f2 = dance();

      futures::join!(f1, f2);
  }
  ```
