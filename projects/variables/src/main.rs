fn main() {
    // println!("안녕하세요!");
    // another_function();
    // another_function(5);
    another_function(5,6);
}

fn another_function(x: i32, y: i32) {
    // println!("또 다른 함수!");
    println!("x의 값 : {}", x);
    println!("y의 값 : {}", y);
}

/*
fn variables() {

    /* 변수 선언 
    let mut x = 5; // mut 키워드를 사용하면 가변 변수를 선언할 수 있음
    println!("x의 값 : {}", x);

    x = 6;
    println!("x의 값 : {}", x);
    */

    /* 상수 선언
    const MAX_POINTS: u32 = 100_000;  // _ 구분은 숫자 리터럴 가독성을 높이기 위해 사용되기도 함
    println!("MAX POINTS : {}", MAX_POINTS);
    */

    /* 변수 가리기
    let x = 5;
    let x = x+1;
    let x = x*2;
    println!("x의 값 : {}", x)

    let spaces = "    ";
    let spaces = spaces.len();  // 정상!! 

    let mut spaces = "    "; 
    spaces = spaces.len();  // 에러 발생!!
    */

    /* 부동 소수점 타입
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    */

    /* 사칙연산
    let sum = 5 +10;  // 덧셈
    let difference = 99.5-4.3; // 뺄셈
    let product = 4*30; // 곱셈
    let quotient = 56.7 / 32.2; // 나눗셈
    let remainder = 43 % 5; // 나머지
    */

    /* 불리언 타입
    let t = true;
    let f: bool = false; // 타입 어노테이션을 사용하는 경우
    */

    /* 문자 타입
    let c = 'z';  
    */


    // 3.2.2 컴파운드 타입
    
    /* 튜플 타입
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("y의 값 : {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);
    */

    /* 배열 타입
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"];
    
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5];

    let first = a[0];
    let second = a[1];
    */

}
*/