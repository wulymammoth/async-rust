#![feature(async_await, futures_api)]
use futures::{Future, join};
use std::thread;

#[allow(unused_must_use)]
fn main() {
    get_two_sites();
    get_two_sites_async();
}

fn get_two_sites() {
    // spawn two threads to do work
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // wait for both threads to complete
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

fn download(url: &str) {
    println!("{}", url);
}

async fn get_two_sites_async() {
    // create two different "futures" which, when run to complettion, will asynchronously download
    // the webpages
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // run both futures to completion at the same time
    join!(future_one, future_two);
}

async fn download_async(url: &str) {
    println!("{}", url);
}
