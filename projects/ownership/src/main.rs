fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    
    //
    let a = String::from("hello, world");
    let rst = first_word(&a);
    println!("{}", rst);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}", item);
        if item == b' ' {
            return i;
        }
    }
    s.len()
}