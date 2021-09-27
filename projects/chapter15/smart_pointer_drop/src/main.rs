struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다!", self.data);
    }
}

/*
fn main() {
    let c = CustomSmartPointer { data: String::from("내 데이터") };
    let d = CustomSmartPointer { data: String::from("다른 데이터") };
    println!("CustomSmartPointer를 생성했습니다.");
}
*/

fn main() {
    let c = CustomSmartPointer { data: String::from("내 데이터") };
    println!("CustomSmartPointer를 생성했습니다.");
    // c.drop(); // error 발생 : CustomSmartPointer 구조체의 drop 메소드를 직접 호출할 수 없음
    drop(c);
    println!("CustomSmartPointer를 main 함수의 끝에 도달하기 전에 해제합니다.");
}