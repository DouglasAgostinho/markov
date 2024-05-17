
use std::io;


fn get_num() -> f32{
    
    let mut num: String = String::new();

    match io::stdin().read_line(&mut num) {

        Ok(n) => n,
        Err(e) => {
            println!("The value inserted {}, will cause Error: {}. Num will be set to 0.0 (zero)", num, e);
            return 0.0;
        }        
    };

    return match num.trim().parse::<f32>() {
        Ok(i) => i,
        Err(e) => {
            println!("The value inserted {}, caused the Error {}. Num will be set to 0.0 (zero)", num, e);
            return 0.0;
        }        
    };
}

fn main() {

    println!("Testing function get num {}", get_num());
    
}
