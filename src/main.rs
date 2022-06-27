fn main() {
    get_data_type();
    // get_shadowing();
}

fn get_data_type() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
}

// https://rinthel.github.io/rust-lang-book-ko/ch03-02-data-types.html
fn get_shadowing() {
    // shadowing. 매번 자료형이 다를 수 있다.
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);
}
