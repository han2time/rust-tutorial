fn main() {

    /* Loop 이용
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter *2
        }
    };
    println!("The result is {}", result);
    */

    /* while을 이용한 조건 루프
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("발사!");
    */


    // for를 이용해 컬렉션을 반복 처리하기
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("요소의 값 : {}", element);
    }


}
