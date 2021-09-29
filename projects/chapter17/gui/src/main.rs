use gui::{Draw, Button, Screen};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>, 
}

impl Draw for SelectBox {
  fn draw(&self) {
    // SelectBox를 그리는 코드
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("예"),
          String::from("아니요"),
          String::from("모름"),
        ],
      }),
      
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("확인"),
      }),
    ],

  };
  screen.run();
}