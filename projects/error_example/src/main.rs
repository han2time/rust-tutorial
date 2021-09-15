// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
// use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

use std::error::Error;

/*
fn main() {
    // panic!("crash and burn");

    // let f: u32 = File::open("hello.txt");
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("파일 열기 실패: {:?}", error);
    //     }
    // };

    /* match 표현식으로 여러 종류의 에러 처리하기
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
            },
            other_error => panic!("파일을 열지 못했ㅅ브니다: {:?}", other_error),
        }
    };
    */

    // 에러 발생 시 패닉을 발생하는 더 빠른 방법: unwrap과 expect
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("파일을 열 수 없습니다.");
}
*/

/* match 표현식을 이용해 호출자에게 에러를 리턴하는 함수
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
*/

/* ? 연산자를 이용해 호출자에게 에러를 리턴하는 함수
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s);
    // Ok(s)

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
*/

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")?;
// }


fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");
    Ok(())
}
