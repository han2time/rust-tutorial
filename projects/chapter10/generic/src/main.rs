/* 10.1
fn largest(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  
  let result = largest(&number_list);
  println!("가장 큰 숫자: {}", result);


  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  
  let result = largest(&number_list);
  println!("가장 큰 숫자: {}", result);

}
*/

/* 10.2.1
fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  
  let result = largest_i32(&number_list);
  println!("가장 큰 숫자: {}", result);


  let char_list = vec!['y', 'm', 'a', 'q'];
  
  let result = largest_char(&char_list);
  println!("가장 큰 문자: {}", result);
}
*/

/* 10.2.2
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.0, y: 4.0};
  let integer_and_float = Point { x: 5, y: 4.0};   
}
*/

/* ex. 10-9
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
*/

/* 10.2.4
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point { x:5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };
  
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
*/

/* 10.2.5
enum Option_i32 {
  Some(i32),
  None,
}
enum Option_f64 {
  Some(f64),
  None,
}

fn main() {
  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
}
*/

/*
mod lib;
use lib::trait_example;
use lib::trait_example::Summary;
use lib::trait_example::Tweet;
use lib::trait_example::NewsArticle;


fn main() {
  // let article = NewsArticle {
    //   headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
    //   location: String::from("카잔 아레나, 러시아"),
    //   author: String::from("위키백과"),
    //   content: String::from("2018년 6월 27일 러시아 카잔의 ... 2:0 승리를 거뒀다.")
    // };
    // println!("새로운 기사: {}", article.summarize());
    
    let tweet = Tweet {
        userName: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    };
    println!("새 트윗 1개: {}", tweet.summarize());
    
    trait_example::notify(tweet);


}
*/

/* ex. 10-15
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("가장 큰 숫자: {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];  
  let result = largest(&char_list);
  println!("가장 큰 문자: {}", result);
}
*/ 

use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {
      x,
      y,
    }
  }
}

impl<T: Display + PartialOrd> Pair<T>{
  fn cmp_display(&self) {
    if self.x > self.y {
      println!("가장 큰 멤버는 x: {}", self.x);
    } else {
      println!("가장 큰 멤버는 y: {}", self.y);
    }
  }
}


fn main() {
  let s = 3.to_string();
}