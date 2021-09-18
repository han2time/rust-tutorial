
pub mod trait_example {
  // pub trait Summary {
  //   fn summarize(&self) -> String;
  // }
  pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
      format!("{} 님의 기사 더 읽기", self.summarize_author())
    }

    // fn summarize(&self) -> String {
    //   String::from("(계속 읽기)")
    // }

  }
  
  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  
  // impl Summary for NewsArticle {
  //   fn summarize(&self) -> String {
  //     format!("{}, by {}, ({})", self.headline, self.author, self.location)
  //   }
  // }
  
  pub struct Tweet {
    pub userName: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }
  
  impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //   format!("{}: {}", self.userName, self.content)
    // }
    
    fn summarize_author(&self) -> String {
      format!("@{}", self.userName)
    }

  }

  pub fn notify(item: impl Summary) {
    println!("속보! {}", item.summarize());
  }

}
