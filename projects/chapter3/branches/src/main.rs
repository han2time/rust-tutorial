fn main() {
    
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("number의 값: {}", number);

    // ------------------------------
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("변수가 4로 나누어떨어집니다.")
    // } else if number % 3 == 0 {
    //     println!("변수가 3으로 나누어떨어집니다.")
    // } else if number % 2 == 0 {
    //     println!("변수가 2로 나누어떨어집니다.")
    // } else {
    //     println!("변수가 4, 3, 또는 2로 나누어 떨어지지 않습니다.")
    // };


    // ------------------------------
    // let number = 3;

	// if number < 5 {
	// 	println!("조건이 일치합니다.");
	// } else {
	// 	println!("조건이 일치하지 않습니다.");
	// }

    // 불리언이 아닌 값을 불리언으로 변환해주지 않음
    // if number {
    //     println!("변수에 저장된 값은 3입니다.");
    // }

    


}
