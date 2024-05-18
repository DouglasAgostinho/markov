
use std::io;


struct Vessel {
        
    _size: f32,
    in_valve: f32,
    out_valve: f32,
    level: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    setpoint: f32,
    time: f32,
    integral: f32,
    time_prev: f32,
    e_prev: f32,
}

impl Vessel{

    fn in_flow(&self, flow: f32) -> f32{
        return flow * (self.in_valve / 100.0);
    }

    fn out_flow(&self) -> f32{
        return self.out_valve * (self.level / 1000.0);
    }

    fn pid_mv (&mut self, i: &f32) -> f32 {
        
        self.time = i * 0.1;
        
        //Value used when error is 0
        let offset: f32 = 0.0;
        
        //calculations start
        
        let e: f32 = self.setpoint - self.level;
        println!("e = {}", e);
        
        let p: f32 = self.kp * e;
        println!("p = {}", p);
        
        self.integral = self.integral + (self.ki * e * (self.time - self.time_prev));
        println!("integral = {}", self.integral);
        
        let d = self.kd * (e - self.e_prev) / (self.time - self.time_prev);
        println!("d = {}", d);
        
        self.out_valve = (offset + p + self.integral + d) * -1.0;
        
        self.e_prev = e;
        self.time_prev = self.time;
        
        //self.level = self.level + self.out_valve;

        if self.out_valve > 99.9 {
            self.out_valve = 100.0
        }
        else if self.out_valve < 0.01 {
            self.out_valve = 0.0
            
        }
        
        return self.out_valve;

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
        _size: 3200.0,
        in_valve: 0.0,
        out_valve: 0.0,
        level: 1600.0,   
        kp: 0.005,
        ki: 0.002,
        kd: 0.005,
        setpoint: 500.0,
        time: 0.0,
        integral: 0.0,
        time_prev: 0.0,
        e_prev: 0.0,     
    };

    println!("Insert the valve opening value");
    water_vessel.in_valve = get_num(water_vessel.in_valve);    

    
    let mut act_mv: f32 = 0.0;

    for i in 1..240{

        let in_flow: f32 = water_vessel.in_flow(25.0);

        let out_flow: f32 = water_vessel.out_flow();
    
        water_vessel.level = water_vessel.level + in_flow - out_flow;
        
        let i: f32 = i as f32;
        
        println!("MV is {:.3} & level is {:.3}", act_mv, water_vessel.level);
        
        act_mv = water_vessel.pid_mv(&i);

      

        println!("\n");
        println!("In Flow is {:.2} & Out Flow is {:.2}", in_flow, out_flow);
        println!("Level is {:.2} & Setpoint is {:.2}", water_vessel.level, water_vessel.setpoint);    
        println!("Out Valve is {:.2}", water_vessel.out_valve);
        println!("\n");

        if (water_vessel.level < water_vessel.setpoint + 1.0) && (water_vessel.level > water_vessel.setpoint -1.0) {
            break;
        }
        
    }    
    
    
}
