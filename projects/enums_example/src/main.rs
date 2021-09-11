/*
enum IpAddrKind {
    V4,
    V6
}

fn route(ip_Type: IpAddrKind) { }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
*/

// 개별 값을 각기 다른 타입으로 정의한 Message 열거자
#[derive(Debug)]
enum Message {
    Quit, // 연관 데이터를 전혀 갖지 않는다
    Move { x: i32, y: i32 }, // 익명 구조체를 포함한다
    Write(String), // 하나의 String 값을 포함한다
    ChangeColor(i32, i32, i32), // 세 개의 i32 값을 포함한다
}

impl Message {
    fn call(&self){
        // 여기에 메서드 본문 작성
        println!("{:?}", &self);
    }
}


fn main() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number : Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; // Error! 


    // let m = Message::Write(String::from("hello"));
    // m.call()


    // 구조체를 이용해 IP 주소의 종류와 데이터 저장
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopbak = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopbak = IpAddr::V6(String::from("::1"));

    // let home = IpAddr::V4(127, 0, 0, 8);
    // let loopbak = IpAddr::V6(String::from("::1"));

}