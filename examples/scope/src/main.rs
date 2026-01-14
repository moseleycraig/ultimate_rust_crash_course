fn main() {
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    } // block ends and y value dropped
     println!("{}, {}", x, y); //Error!   
}
