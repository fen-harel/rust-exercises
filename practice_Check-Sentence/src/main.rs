// Take input and check if said input is a "proper" sentence.

fn get_input() -> String {

    println!("Please enter the sentence to be :");

    let mut input_string: String = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed");
    return input_string;
}

fn first_letter(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if i == b' ' {          // b' ' = byte literal
            return i;
        }
    }

    s.len()
}

enum UpperCase {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

fn main() {
    let sentence = get_input();
    let upper = sentence.to_uppercase();    // type String
    
    let mut s = String::from(sentence);

    let word = first_letter(&s);

    println!("{}", word);

    // println!("This is upper ---> {}", upper);
    // println!("This is input_string ~~> {}", sentence);
}