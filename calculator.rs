use std::io::stdin;

fn main() {
    println!("Please enter first number: ");
    let first = get_input().trim().parse::<f32>().unwrap();
    
    println!("Choose between:\n1)Addition\n2)Subtraction\n3)Multiplication\n4)Division");
    let func = get_input().trim().parse::<i32>().unwrap();
    
    println!("Please enter second number: ");
    let second = get_input().trim().parse::<f32>().unwrap();

    let output = match func{
        1=>first+second,
        2=>first-second,
        3=>first*second,
        4=>first/second,
        _=>0.0
    };

    println!("{}", output);
}

fn get_input() -> String {
    let mut ret_val = String::new();
    stdin().read_line(&mut ret_val).expect("Failed to read input");
    return ret_val
}
