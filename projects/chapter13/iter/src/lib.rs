/*
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

}

#[cfg(test)]
mod test {
  #[test]
  fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
  }

  #[test]
  fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
  }

}
*/

/* 13.2.4 환경을 캡처하는 클로저의 활용
#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}

#[test]
fn filters_shoe() {
    let shoes = vec![
      Shoe { size: 10, style: String::from("스니커즈") },
      Shoe { size: 13, style: String::from("샌달") },
      Shoe { size: 10, style: String::from("부츠") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
      in_my_size,
      vec![
        Shoe { size: 10, style: String::from("스니커즈") },
        Shoe { size: 10, style: String::from("부츠") },
      ]
    )
}
*/

// 13.2.5 Iterator 트레이트를 이용해 직접 반복자 구현하기
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;

    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

 // 13.2.7 Iterator 트레이트의 다른 메서드 활용하기
 #[test]
 fn using_other_iterator_trait_methods() {
     let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                  .map(| (a, b) | a*b)
                                  .filter(|x| x%3 == 0)
                                  .sum();
      assert_eq!(18, sum);
 }