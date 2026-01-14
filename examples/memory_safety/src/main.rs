fn main() {
    let enigma: i32;
    println!("{}, enigma); // declared, but not initialized to a value before use, won't compile!
}

// error[E0381]: use of possibly uninitialized variable: 'enigma'
