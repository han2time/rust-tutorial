fn main() {
    // let mut s = String::new(); // 빈 문자열 생성
    // let data = "문자열초깃값";
    // let s = data.to_string();

    // 문자열 리터럴의 to_string() 메서드를 직접 호출할 수 있다.
    // let s = "문자열초깃값".to_string();

    // let s = String::from("문자열초깃값");


    /* 문자열 수정하기
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2);
    */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 이렇게 하면 변수 s1은 메모리가 해제되어 더 이상 사용할 수 없다.
    println!("{}", s3);


    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toc");
    let s = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s);


    let hello = "안녕하세요";
    // let sl = &hello[0..3];
    // println!("{}",sl);
    for c in hello.chars() {
        println!("{}", c);
    }

    
}
