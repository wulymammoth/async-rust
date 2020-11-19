// `block_on` blocks the current thread until the provided future has run to completion. Other
// executors provide more complex behavior, like scheduling multiple futures onto the same thread
use futures::executor::block_on;

pub fn main() {
    println!("\n1.3 async/.await primer\n");

    println!("[async hello world]\n");
    let future = hello_world(); // nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed

    println!("\n[block party] | not very performant\n");
    block_party();

    println!("\n[sing and dance at the same time (concurrently)]\n");
    block_on(async_main());
}

async fn hello_world() {
    println!("hello, world!");
}

async fn learn_song() {
    println!("learn song (function)");
}

async fn sing_song() {
    println!("sing song (function)");
}

async fn dance() {
    println!("dance (function)");
}

// one way to learn, sing, and dance is to block on each individually
fn block_party() {
    block_on(learn_song());
    block_on(sing_song());
    block_on(dance());
}

// let's sing and dance at the same time!
async fn learn_and_sing() {
    // wait until the song has been learned before singing it
    // we use `.await` here rather than `block_on` to prevent blocking the thread, which makes it
    // possible to `dance` at the same time
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance` future will take
    // over the current thread. If `dance` becomes blocked, `learn_and_sing` can take back over. If
    // both futures are blocked, then `async_main` is blocked and will yield to the executor
    futures::join!(f1, f2);
}
