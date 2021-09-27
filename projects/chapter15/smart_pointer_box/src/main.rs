use crate::List::{Cons, Nil};

/* 
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);    
}
*/


struct MyBox<T>(T) ;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
/*
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);    
}
*/

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("안녕하세요 {}!", name);
}

/*
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
*/

/*
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("b를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    }
    println!("c가 범위를 벗어난 후의 카운터 = {}", Rc::strong_count(&a));
}
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;

    println!("a 수정 후 = {:?}", a);
    println!("b 수정 후 = {:?}", b);
    println!("c 수정 후 = {:?}", c);
}