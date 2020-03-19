use async_std::task;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::time::Instant;

// Fetch the HTML contents of a web page.
async fn get(url: &str) -> String {
    surf::get(url).recv_string().await.unwrap()
}

use std::io::{self, Read};

fn main() {
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    task::block_on(async {
        let start = Instant::now();
        let mut tasks = Vec::new();
        // Fetch the list of contributors for the first 40 minor Rust releases.
        for i in 0..40 {
            let url = format!("https://thanks.rust-lang.org/rust/1.{}.0/", i);
            // Spawn a task fetching the list.
            tasks.push(task::spawn(async move {
                let html = get(&url).await;
                // Display the number of contributors to this Rust release.
                for line in html.lines() {
                    if line.contains("individuals") {
                        println!("{}", line.trim());
                    }
                }
            }))
        }
        // Wait for all tasks to complete.
        for t in tasks {
            t.await;
        }
        // Display elapsed time.
        dbg!(start.elapsed());
    });
}
