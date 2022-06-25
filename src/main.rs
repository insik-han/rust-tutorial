use std::env::{args, Args};

fn main() {
    get_args();
}

// fn data_type() {
// let a = 2;
// a = 3; // error

// 변수의 아이템을 가져오거나 변수의 값을 바꾸려면 mut 선언 필요
// let mut b = 3;
// b = 4;
// }

fn get_args() {
    // 환경변수 조회
    // cargo run -- abc => { inner: ["target/debug/rust-tutorial", "abc"]} 환경변수 cli에서 삽입 가능
    let mut args: Args = args();

    // Some: None의 가능성을 가지고 있다.
    // None: 없다
    // .unwrap(): Option의 경우를 제거. 안전하지는 않다. None이 되는 상황에 unwrap()을 하면 타입 에러
    // .nth(n): interable의 n번쨰 아이템을 리턴 후 iter.next()
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);

    // println! => 내장함수! 형태를 macros라 한다.
    // println!("{}", variables); // 원시타입은 {}로 삽입
    // println!("{:?}", variables); // 오브젝트형은 {:?}로 삽입
    // print!("{:?}", args);
    println!("{}", result);
}

fn operate(opeator: char, first_number: f32, second_number: f32) -> f32 {
    if opeator == '+' {
        first_number + second_number
    } else if opeator == '-' {
        first_number - second_number
    } else if opeator == '/' {
        first_number / second_number
    } else if opeator == '*' {
        first_number * second_number
    } else {
        0.0
    }
}
