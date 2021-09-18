fn main() {
  /* 범위를 벗어난 값 참조 시도
  let r;
  {
    let x = 5;
    r = &x;
  }
  println!("r: {}", r);
  */
  
  /*
  let string1 = String::from("abcd");
  let string2 = "xyz";
  
  let result = longest(string1.as_str(), string2);
  println!("더 긴 문자열: {}", result);
  */

  let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에.");
  let first_sentence = novel.split('.')
    .next()
    .expect("문장에서 마침표 '.'를 찾을 수 없습니다.");
  let i = ImportantExcerpt { part: first_sentence };

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}


struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }

  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("주목해 주세요! {}", announcement);
    self.part
  }
}