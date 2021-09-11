#[derive(Debug)]
enum UsState {
    Alabama, Alaska,
    // -- 생략 --
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(&coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);

    // 자리지정자 _
    // let some_u8_value = 0u8;
    // match some_u8_value {
    //     1 => println!("one!"),
    //     3 => println!("three!"),
    //     5 => println!("five!"),
    //     7 => println!("seven!"),
    //     _ => (),
    // }

    // if let을 이용한 간결한 흐름 제어
    // let some_u8_value = Some(0u8);
    // if let Some(3) = some_u8_value {
    //     println!("three")
    // }

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("{:?} 주의 25센트 동전!", state),
        _ => count+=1,
    }
}


