
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }
            fn cook_order() {}
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    /*
    // 절대경로
    crate::front_of_house::hosting::add_to_waitlist();
    // 상대경로
    front_of_house::hosting::add_to_waitlist();
    */

    // 여름에 아침 식사로 호밀빵을 주문한다
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    // 고객이 마음을 바꿔서 빵 종류를 바꾼다
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // 다음 줄의 주석을 해제하면 컴파일 되지 않는다
    // meal.seasonal_fruit = String::from("블루베리");
    

    // 열거자르 공개하면 모든 열것값도 공개된다
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
*/

/*
// USE
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
*/


// 모듈을 다른 파일로 분류하기
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


