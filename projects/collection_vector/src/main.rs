#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    /*
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3];

    // 벡터 수정
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v)
    */

    /* 벡터로부터 값 읽기
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("세 번째 원소: {}", third);

    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }
    */

    // 인덱스값 접근 
    // let does_not_exist = &v[100];   // 패닉(panic) 발생
    // let does_not_exist = v.get(100);    // 패닉이 발생하지 않고 None 값이 리턴됨

    // 값을 참조하는 변수를 생성한 상태에서 벡터에 새로운 값을 추가
    // let mut w = vec![1, 2, 3, 4, 5];
    // let first = &w[0];
    // w.push(6); // error 발생!

    /* 벡터에 저장된 값을 순회하기
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;       // * : 역참조 연산자(p385)
        println!("{}", i);
    }
    */

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);


}
