fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 기존의 인스턴스로부터 새 인스턴스 생성하기
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("User1 email : {}", user1.email);
    println!("User2 sign_in_count : {}", user2.sign_in_count);


    // 이름 없는 필드를 가진 튜플 구조체로 다른 타입 생성하기
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
