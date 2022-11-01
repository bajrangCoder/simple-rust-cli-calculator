use std::io;

fn main() {
    println!("Welcome to cli calculator!");
    
    let mut first_no = String::new();
    println!("Enter first number: ");
    io::stdin().read_line(&mut first_no).expect("failed to read from stdin");
    let first_no: i32=first_no.trim().parse().unwrap();
    
    let mut second_no = String::new();
    println!("Enter second number: ");
    io::stdin().read_line(&mut second_no).expect("failed to read from stdin");
    let second_no: i32=second_no.trim().parse().unwrap();
    
    let mut opt = String::new();
    println!("Enter operation (sum : 1, subtract : 2,multiply : 3,divide : 4) : ");
    io::stdin().read_line(&mut opt).expect("failed to read from stdin");
    let opt: i32=opt.trim().parse().unwrap();
    
    if opt == 1 {
        println!("{} + {} = {}",first_no,second_no,(first_no+second_no));
    } else if opt == 2 {
        println!("{} - {} = {}",first_no,second_no,(first_no-second_no));
    } else if opt == 3 {
        println!("{} * {} = {}",first_no,second_no,(first_no*second_no));
    } else if opt == 4 {
        println!("{} ÷ {} = {}",first_no,second_no,(first_no/second_no));
    } else {
        println!("Invalid Option!");
    }
}
