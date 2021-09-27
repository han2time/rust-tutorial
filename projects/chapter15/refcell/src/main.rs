use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match *self {
      Cons(_, ref item) => Some(item),
      Nil => None,
    }
  }
}

fn main() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  println!("a의 최초 rc카운트 = {}", Rc::strong_count(&a));
  println!("a의 다음 아이템 : {:?}", a.tail());

  let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

  println!("b를 생성한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
  println!("b의 최초 rc카운트 = {}", Rc::strong_count(&b));
  println!("b의 다음 아이템 : {:?}", b.tail());

  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  println!("a를 변경한 후 b의 rc 카운트 = {}", Rc::strong_count(&b));
  println!("a를 변경한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));

  // 순환 참조가 생성된 것을 확인하려면 다음 코드의 주석을 해제하면 됨
  // 하지만 그렇게 하면 스택 오버플로가 발생함
  // println!("a의 다음 아이템 = {:?}", a.tail());

}