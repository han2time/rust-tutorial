/*
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("테스트 실패!");
    }
}
*/

/* 11.1.2
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
*/

// 11.1.3
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}


/* 11.1.4
pub fn greeting(name: &str) -> String {
    // format!("안녕하세요 {}!", name)
    format!("안녕하세요!")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("캐롤");
        assert!(
            result.contains("캐롤"),
            "Greeting 함수의 결과에 이름이 포함되어 있지 않음. 결괏값: '{}'", result
        );
    }
}
*/

/* 11.1.5
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("반드시 1과 100 사이의 값을 사용해야합니다. 지정된 값: {}", value);
        // }
        if value < 1 {
            panic!("반드시 1보다 크거나 같은 값을 사용해야 합니다. 지정된 값: {}", value);
        } else if value > 100 {
            panic!("반드시 100보다 작거나 같은 값을 사용해야 합니다. 지정된 값: {}", value);
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "반드시 100보다 작거나 같은 값을 사용해야 합니다.")]
    fn generate_than_100() {
        Guess::new(200);
    }
}
*/