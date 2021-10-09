use std::slice;

fn main() {
    /* 19.1.2
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe{
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }
    */

    /* 19.1.3
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);
        // (&mut slice[..mid], &mut slice[mid..])  // Error!!

        let ptr = slice.as_mut_ptr();
        unsafe {
            (slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }

    // 외부함수 호출
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("C 언어에 따르면 -3의 절댓값은 {}입니다.", abs(-3));
    }

    // 외부함수로 인터페이스
    pub extern "C" fn call_from_c() {
        println("러스트 함수를 C에서 호출했습니다!");
    }
    */
    
    // 19.1.4
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}

// 19.1.4
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}