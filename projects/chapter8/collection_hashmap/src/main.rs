use std::collections::HashMap;


fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("블루"), 10);
    // scores.insert(String::from("옐로"), 50);
    
    // let teams = vec![String::from("블루"), String::from("옐로")];
    // let initial_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // // let team_name = String::from("블루");
    // // let score = scores.get(&team_name); // Options<&V> 타입 리턴

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
    // println!("{:?}", scores);

    /* entry 메서드를 이용해서 키에 값이 할당되어 있지 않을 때만 값 추가
    let mut scores = HashMap::new();
    scores.insert(String::from("블루"), 10);

    scores.entry(String::from("옐로")).or_insert(50);
    scores.entry(String::from("블루")).or_insert(50);

    println!("{:?}", scores);
    */

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
