use reqwest::Client as Reqwest;
use std::error::Error;
use std::thread;

// create 100 threads
// for each thread download some contentt from a webpage

// fn main() {
//     // let process = thread::spawn(move || {
// //         for i in 0..V100 {
// //             std::thread::spawn(move || {
// //                 let resp = Reqwest::get(
// //                     "curl -L https://crates.io/api/v1/crates/$1/$2/download | tar -xf -
// // mv $1-$2",
// //                 )
// //                 .unwrap()?;
// //                 println!("Thread number {}", i);
// //             });
// //         }
// //     });
//     // println!("Hello, world!");
// }
//  print the webpage threads message 100 times.
fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    //creating 100 threads
    let mut handles = vec![];

    for i in 0..100 {
        // Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Each thread performs an HTTP GET request
            let response = reqwest::blocking::get("https://httpbin.org/ip")
                .unwrap()
                .text()
                .unwrap();
            println!("Thread number {}", i);
            println!("{}", response);
        });

        handles.push(handle); // Store the thread handle
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:#?}", resp);
    Ok(())
}
