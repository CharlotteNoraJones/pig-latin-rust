fn main() {
    let mut s1: String = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("{}", len);

    change(&mut s1);

    println!("{}", s1);

    let mut s2: String = String::from("test");

    let r1: &String = &s2;
    let r2: &String = &s2;

    println!("{}, {}", r1, r2);

    let r3: &mut String = &mut s2;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
