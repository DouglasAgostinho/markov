
use std::io;
use std::{thread, time};

fn get_max (table: [[f32; 7]; 7000]) -> i32 {

    let mut max: f32 = 0.0;
    let mut index: i32 = 0;

    for i in 0..7000{

        if table[i][6] > max {
            max = table[i][6];
            index = i as i32;
        }
    }
    return index;
}

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
    reward: f32,
    train_table: [[f32; 7]; 7000],
    _q_table: [[i32; 7]; 20000],

}

impl Vessel{

    fn in_flow(&self, flow: f32) -> f32{
        return flow * (self.in_valve / 100.0);
    }

    fn out_flow(&self) -> f32{
        return self.out_valve * (self.level / 1000.0);
    }

    fn pid_mv (&mut self, i: &f32){
        
        self.time = i * 0.1;
        
        //Value used when error is 0
        let offset: f32 = 0.0;
        
        //calculations start

        //println!("\n -----{}-----", i);
        
        let e: f32 = self.setpoint - self.level;
        //println!("e = {}", e);
        
        let p: f32 = self.kp * e;
        //println!("p = {}, kp= {}", p, self.kp);
        
        //println!("integral = {}, ki = {}, time = {}, prev_time = {}", self.integral, self.ki, self.time, self.time_prev);
        self.integral = self.integral + (self.ki * e * (self.time - self.time_prev));
        
        
        let d = self.kd * (e - self.e_prev) / (self.time - self.time_prev);
        //println!("d = {}, kd = {}", d, self.kd);
        
        self.out_valve = (offset + p + self.integral + d) * -1.0;
        //println!("Out valve = {}", self.out_valve);
        
        self.e_prev = e;
        self.time_prev = self.time;
        
        //println!("\n -----{}-----", i);

        //self.level = self.level + self.out_valve;

        if self.out_valve > 99.9 {
            self.out_valve = 100.0
        }
        else if self.out_valve < 0.01 {
            self.out_valve = 0.0
            
        }
        
        //return self.out_valve;

    }

    fn run(&mut self){

        for i in 1..10000{

            println!("level is {:.3}", self.level);

            let in_flow: f32 = self.in_flow(self.in_valve);
    
            let out_flow: f32 = self.out_flow();
        
            self.level = self.level + in_flow - out_flow;
            
            let i: f32 = i as f32;
            
            println!("MV is {:.3} & level is {:.3}", self.out_valve, self.level);
            
            self.pid_mv(&i);      
    
            println!("\n -----{}-----", i);
            println!("In Flow is {:.2} & Out Flow is {:.2}", in_flow, out_flow);
            println!("Level is {:.2} & Setpoint is {:.2}", self.level, self.setpoint);    
            println!("Out Valve is {:.2}", self.out_valve);
            println!(" -----{}-----\n", i);
    
            /*if (self.level < self.setpoint + 1.0) && (self.level > self.setpoint -1.0) {
                break;
            }*/
            thread::sleep(time::Duration::from_millis(250));
        } 
    }

    fn train(&mut self){
        /*  Train table will be as follow
            0 - in_flow,
            1 - level,
            2 - out_flow,
            3 - p,
            4 - i,
            5 - d,
            6 - reward,
        */        

        let in_flow = self.in_flow(self.in_valve);
        let level = self.in_flow(self.level);
        let out_flow = self.out_flow();
        
        for i in 1..7000{

            self.level = 1600.0;
            self.integral = 0.0;
            self.time_prev = 0.0;

            self.train_table[i][0] = in_flow;
            self.train_table[i][1] = level;
            self.train_table[i][2] = out_flow;            

            self.train_table[i][3] = self.train_table[i - 1][3] + 0.001;   
            self.train_table[i][4] = self.train_table[i - 1][4];
            self.train_table[i][5] = self.train_table[i - 1][5];

            if self.train_table[i][3] > 0.019 {

                self.train_table[i][4] = self.train_table[i - 1][4] + 0.001;
                self.train_table[i][3] = 0.001;
            }                            

            if self.train_table[i][4] > 0.019 {

                self.train_table[i][5] = self.train_table[i - 1][5] + 0.001;
                self.train_table[i][4] = 0.001;

            }         

            self.reward = 1000.0;
            self.kp = self.train_table[i][3];
            self.ki = self.train_table[i][4];
            self.kd = self.train_table[i][5];

            for j in 1..400 {

                let in_flow: f32 = self.in_flow(self.in_valve);
        
                let out_flow: f32 = self.out_flow();
            
                self.level = self.level + in_flow - out_flow;
                
                let j: f32 = j as f32;                                
                
                self.pid_mv(&j);      

                /*
                println!("\n -----{}-----", j);
                println!("In Flow is {:.2} & Out Flow is {:.2}", in_flow, out_flow);
                println!("Level is {:.2} & Setpoint is {:.2}", self.level, self.setpoint);    
                println!("Out Valve is {:.2}", self.out_valve);
                println!(" -----{}-----\n", j);
                */

                self.reward = self.reward - 0.5;
        
                if (self.level > self.setpoint + 2.0) || (self.level < self.setpoint -2.0) {
                    self.reward = self.reward - 0.1;
                }                                   

            }

            //thread::sleep(time::Duration::from_millis(250));
            //println!("-------------------------------------------");
            //println!("-------------------------------------------");


            self.train_table[i][6] = self.reward;            
            
        }
        
        println!("{:?}", self.train_table);
        //let xess = get_max(self.train_table);
        //println!("{}", xess);
        //println!("{}", self.train_table[xess as usize][6]);
        //println!("{:?}", self.train_table[xess as usize]);
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
        reward: 0.0,  
        train_table: [[0f32; 7]; 7000],
        _q_table: [[0i32; 7]; 20000],
    };

    //let mut q_table: [[i32; 7]; 20000] = [[0i32; 7]; 20000];

    println!("Insert the valve opening value");
    water_vessel.in_valve = get_num(water_vessel.in_valve);                
    
    water_vessel.train();

    let xess = get_max(water_vessel.train_table);
    println!("{}", xess);
    println!("{:?}", water_vessel.train_table[xess as usize]);

    water_vessel.kp = water_vessel.train_table[xess as usize][3];
    water_vessel.ki = water_vessel.train_table[xess as usize][4];
    water_vessel.kd = water_vessel.train_table[xess as usize][5];

    water_vessel.level = 1600.0;
    water_vessel.integral = 0.0;
    water_vessel.time_prev = 0.0;

    water_vessel.run();

    let xess = get_max(water_vessel.train_table);
    println!("{}", xess);
    println!("{:?}", water_vessel.train_table[xess as usize]);
    
}
