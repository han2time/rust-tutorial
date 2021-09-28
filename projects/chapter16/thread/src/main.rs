use std::thread;
use std::time::Duration;

/* 16.1.2
fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("새 스레드: {}", i);
    //         thread::sleep(Duration::from_millis(1))
    //     }
    // });
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("새 스레드: {}", i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    
    handle.join().unwrap();

    for i in 1..5 {
        println!("주 스레드: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
}
*/

/* 16.1.3
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("벡터: {:?}", v);
    });

    handle.join().unwrap();
}
*/

use std::sync::mpsc;

/* 16.1.5
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("안녕하세요");
        tx.send(val).unwrap();
    });

    let recived = rx.recv().unwrap();
    println!("수신: {}", recived);
}
*/

/* 16.1.6
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("자식 스레드가"),
            String::from("안녕하세요"),
            String::from("라고"),
            String::from("인사합니다."),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("수신: {}", recieved);
    }
}
*/

// 16.1.7
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("자식 스레드가"),
            String::from("안녕하세요"),
            String::from("라고"),
            String::from("인사합니다."),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("그리고"),
            String::from("더 많은"),
            String::from("메세지를"),
            String::from("보냅니다."),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });



    for recieved in rx {
        println!("수신: {}", recieved);
    }
}