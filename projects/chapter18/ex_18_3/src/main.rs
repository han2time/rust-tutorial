fn main() {
    // 18.3.2
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(y) => println!("일치, y = {:?}", y),
        _ => println!("일치하지 않음, x = {:?}", x),
    }
    println!("결과: x = {:?}, y = {:?}", x, y);

    // 18.3.4
    let x1 = 5;
    match x1 {
        1 ..= 5 => println!("1에서 5중 하나"),
        _ => println!("그 외 나머지 값"),
    }

    let x2 = 'c';
    match x2 {
        'a' ..= 'j' => println!("ASCII 문자의 전반부"),
        'k' ..= 'z' => println!("ASCII 문자의 후반부"), 
        _ => println!("그 외 나머지 값"),
    }

    // 18.3.5
    let p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p;
    // assert_eq!(0, a); 
    // assert_eq!(7, b);

    // let Point { x, y } = p;
    // assert_eq!(0, x); 
    // assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("x축 {}에 위치하는 점", x), 
        Point { x: 0, y } => println!("y축 {}에 위치하는 점", y),
        Point { x, y } => println!("좌표 ({}, {})에 위치하는 점", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit: 해제할 값이 없습니다.");
        },
        Message::Move {x, y} => {
            println!(
                "Move: x = {}, y = {}",
                x,
                y
            );
        },
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "ChangeColor: R = {}, G = {}, B = {}",
                r,
                g,
                b
            );
        }
    }

    // 18.3.6 패턴의 값 무시하기
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("이미 설정된 값을 덮어쓸 수 없습니다.");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("현재 설정: {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("일치하는 숫자: {}, {}, {}", first, third, fifth)
        },
    }

    let s = Some(String::from("안녕하세요"));
    if let Some(_s) = s {
        println!("문자열을 찾았습니다.");
    }
    // println!("{:?}", s); // 에러 발생
    
    let origin = Point { x:0, y:0 };
    match origin {
        Point{x, ..} => println!("x = {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("firxt = {}, last = {}", first, last);
        }
    }

    // 18.3.7 매치 가드를 이용한 추가 조건
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("일치, n = {:?}", n),
        _ => println!("일치하지 않음, x = {:?}, y = {:?}", x, y),
    }
    println!("결과: x = {:?}, y = {:?}", x, y);

    // 18.3.8
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3 ...7 } => {
            println!("id를 범위에서 찾았습니다: {}", id_variable);
        },
        Message2::Hello { id: 10...12 } => {
            println!("id를 다른 범위에서 찾았습니다.");
        },
        Message2::Hello { id } => {
            println!("다른 id {}를 찾았습니다.", id );
        },
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    println!("이 함수는 y 매개변수만 사용한다: {}", y);
}

enum Message2 {
    Hello { id: i32 }
}