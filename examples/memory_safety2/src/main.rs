fn main() {
    let enigma: i32;
    if true {
        enigma = 42;
    }
    println!("enigma is {}", enigma); // Error!
    // compiler can't guarantee that enigma is initualized before it's used
    // doesn't know value of true at runtime
}

