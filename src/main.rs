
use std::io;


struct Vessel {
        
    size: f32,
    in_valve: f32,
    out_valve: f32,
    level: f32,
}

impl Vessel{

    fn in_flow(&self, flow: f32) -> f32{
        return flow * (self.in_valve / 100.0);
    }


}

fn get_num(last: f32) -> f32{
    
    let mut num: String = String::new();

    match io::stdin().read_line(&mut num) {

        Ok(n) => n,
        Err(e) => {
            println!("The value inserted {}, will cause Error: {}. Num will be kept in {}", num, e, last);
            return last;  
        }        
    };

    return match num.trim().parse::<f32>() {
        Ok(i) => i,
        Err(e) => {
            println!("The value inserted {}, will cause Error: {}. Num will be kept in {}", num, e, last);
            return last;  
        }        
    };
}

fn main() {

    let mut water_vessel = Vessel{
        size: 1200.0,
        in_valve: 0.0,
        out_valve: 0.0,
        level: 0.0,        
    };

    println!("Insert the valve opening value");
    water_vessel.in_valve = get_num(water_vessel.in_valve);
    
    let flow = water_vessel.in_flow(25.0);

    water_vessel.level = water_vessel.level + flow;
    
    println!("\n\n");
    println!("Flow is {:.2}", flow);
    println!("Level is {:.2}", water_vessel.level);
    println!("Size is {}", water_vessel.size);
    println!("Flow is {}", water_vessel.out_valve);
    println!("\n\n");
    
    
}
