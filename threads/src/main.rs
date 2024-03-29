use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 1..5 {
    //     println!("hi number {} from the main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    //
    // handle.join().unwrap()

    // let v = vec![1, 2, 3];
    //
    // let handle = thread::spawn(move || {
    //     println!("here s a vakavaka {:?}", v);
    // });
    //
    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    //
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("dfa"),
    //         String::from("hsi"),
    //     ];
    //
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("1i"),
    //         String::from("1rom"),
    //         String::from("1fa"),
    //         String::from("1si"),
    //     ];
    //
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // for received in rx {
    //     println!("got {}", received);
    // }

    // let m = Mutex::new(5);
    //
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m)

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *counter.lock().unwrap());
}
