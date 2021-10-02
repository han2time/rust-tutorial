fn main() {
    /* 18.1.2
    let favorite_color : Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("선호하는 {}색을 배경으로 사용합니다.", color);
    } else if is_tuesday {
        println!("화요일엔 녹색이죠!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("보라색을 배경으로 사용합니다.");
        } else {
            println!("오렌지색을 배경으로 사용합니다.");
        }
    } else {
        println!("파란색을 배경으로 사용합니다.");
    }
    */
    
    // 18.1.3 while let condition loop
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 18.1.4 for loop
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("인덱스 {}의 값: {}", index, value);
    }

    // 18.1.6 함수 매개변수
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("현재 위치: ({}, {})", x, y);
}
