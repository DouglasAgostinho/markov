
//Imports
use std::io;

//For Debug and performance monitor
use std::{thread, time};
use std::time::Instant;

//Constant definition
const MAX: usize = 1120;        //Maximum size of training table & Train Loop
const K_MAX: f32 = 0.0101;      //Maximum value of PID K factors (kp, ki, kd) for train loop
const Q_MAX: usize = 1120;


fn arg_max (table: [[f32; 7]; MAX]) -> i32 {
    //Find settings with the higest reward and return index

    let mut max: f32 = 0.0;
    let mut index: i32 = 0;

    //Loop through train table and find the higest reward value
    for i in 0..MAX{
        if table[i][6] > max {
            max = table[i][6];
            index = i as i32;
        }
    }

    //Return higest reward row index
    return index;
}

struct Vessel {
    //Struct for simulation of level control in a liquid container
    //Level will be controlled by a PID controller which will receive continuous adjustments
    //Adjustments will be performed by a pre-trained markov table 
        
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
    train_table: [[f32; 7]; MAX],
    q_table: [[f32; 7]; Q_MAX],
}

impl Vessel{

    fn in_flow(&self, flow: f32) -> f32{
        //Return the input flow based on input valve opening (upgradable to valve opening * pressure)
        return flow * (self.in_valve / 100.0);
    }

    fn out_flow(&self) -> f32{
        //Return outpu flow based on output valve opening and vessel level
        return self.out_valve * (self.level / 1000.0);
    }

    fn pid_mv (&mut self, i: &f32){
        //PID control logic
        
        self.time = i * 0.1;
        
        //Value used when error is 0
        let offset: f32 = 0.0;
        
        //calculations start        

        let e: f32 = self.setpoint - self.level;        
        
        let p: f32 = self.kp * e;        
        
        self.integral = self.integral + (self.ki * e * (self.time - self.time_prev));

        let d: f32 = if i < &2.0 {
            0.0
        }
        else{
            self.kd * (e - self.e_prev) / (self.time - self.time_prev)
        };        
        
        self.out_valve = (offset + p + self.integral + d) * -1.0;        
        
        self.e_prev = e;
        self.time_prev = self.time;                

        if self.out_valve > 99.9 {
            self.out_valve = 100.0
        }
        else if self.out_valve < 0.01 {
            self.out_valve = 0.0
            
        }
    }

    fn run(&mut self){

        for i in 1..10000{

            let in_flow: f32 = self.in_flow(self.in_valve);
    
            let out_flow: f32 = self.out_flow();
        
            self.level = self.level + in_flow - out_flow;
            
            let i: f32 = i as f32;
            
            self.pid_mv(&i);      

            println!("\n -----{}-----", i);
            println!("In Flow is {:.2} & Out Flow is {:.2}", in_flow, out_flow);
            println!("Level is {:.2} & Setpoint is {:.2}", self.level, self.setpoint);    
            println!("Out Valve is {:.2}", self.out_valve);
            println!(" -----{}-----\n", i);
    
            thread::sleep(time::Duration::from_millis(250));
        } 
    }

    fn train(&mut self, lv: f32, iv: f32, ov: f32, k_max: f32, sp: f32){
        /*  Train table will be as follow
            0 - in_flow,
            1 - level,
            2 - setpoint,
            3 - p,
            4 - i,
            5 - d,
            6 - reward,
        */        

        for i in 1..MAX{

            self.train_table[i][0] = iv;
            self.train_table[i][1] = lv;
            self.train_table[i][2] = sp;

            self.train_table[i][3] = self.train_table[i - 1][3] + 0.001;   
            self.train_table[i][4] = self.train_table[i - 1][4];
            self.train_table[i][5] = self.train_table[i - 1][5];

            if self.train_table[i][3] > k_max {

                self.train_table[i][4] = self.train_table[i - 1][4] + 0.001;
                self.train_table[i][3] = 0.001;
            }                            

            if self.train_table[i][4] > k_max {

                self.train_table[i][5] = self.train_table[i - 1][5] + 0.001;
                self.train_table[i][4] = 0.001;

            }         

            if self.train_table[i][5] > k_max {
                break;
            }            

            self.reward = 1000.0;

            self.kp = self.train_table[i][3];
            self.ki = self.train_table[i][4];
            self.kd = self.train_table[i][5];
            self.integral = 0.0;
            self.time_prev = 0.0;

            self.level = lv;
            self.setpoint = sp;
            self.in_valve = iv;
            self.out_valve = ov;            

            for j in 1..400 {

                let in_flow: f32 = self.in_flow(self.in_valve);
                
        
                let out_flow: f32 = self.out_flow();
            
                self.level = self.level + in_flow - out_flow;
                
                let j: f32 = j as f32;                                
                
                self.pid_mv(&j);      

                

                self.reward = self.reward - 0.5;
        
                if (self.level > self.setpoint + 2.0) || (self.level < self.setpoint -2.0) {
                    self.reward = self.reward - 0.1;
                }                                   

            }

            self.train_table[i][6] = self.reward;            
            
        }
        
        //println!("{:?}", self.train_table);
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

    let before: Instant = Instant::now();

    let mut water_vessel = Vessel{
        _size: 3200.0,
        in_valve: 0.0,
        out_valve: 0.0,
        level: 1600.0,   
        kp: 0.005,
        ki: 0.002,
        kd: 0.005,
        setpoint: 1600.0,
        time: 0.0,
        integral: 0.0,
        time_prev: 0.0,
        e_prev: 0.0,   
        reward: 0.0,  
        train_table: [[0f32; 7]; MAX],
        q_table: [[0f32; 7]; Q_MAX],
    };

    //let mut q_table: [[i32; 7]; 20000] = [[0i32; 7]; 20000];
    
    
    water_vessel.train(1600.0, 25.0, 0.0, K_MAX, 500.0);

    

    let xess: i32 = arg_max(water_vessel.train_table);
    //println!("{}", xess);
    

    water_vessel.q_table[xess as usize] = water_vessel.train_table[xess as usize];

    println!(" Elapsed Time: {:?}", before.elapsed());

    println!(" T {:?}", water_vessel.train_table[xess as usize]);
    println!(" Q {:?}", water_vessel.q_table[xess as usize]);

    let sure = arg_max(water_vessel.q_table);

    println!("Sure {sure}");

    

    water_vessel.kp = water_vessel.train_table[xess as usize][3];
    water_vessel.ki = water_vessel.train_table[xess as usize][4];
    water_vessel.kd = water_vessel.train_table[xess as usize][5];

    water_vessel.in_valve = water_vessel.train_table[xess as usize][0];
    water_vessel.level = water_vessel.train_table[xess as usize][1];
    water_vessel.setpoint = water_vessel.train_table[xess as usize][2];

    water_vessel.integral = 0.0;
    water_vessel.time_prev = 0.0;

    

    water_vessel.run();

    //let xess: i32 = arg_max(water_vessel.train_table);
    //println!("{}", xess);
    //println!("{:?}", water_vessel.train_table[xess as usize]);
    
}



//Debugs prints

//------------------------------ PID ------------------------------ 
//println!("\n -----{}-----", i);
//println!("e = {}", e);
//println!("p = {}, kp= {}", p, self.kp);
//println!("integral = {}, ki = {}, time = {}, prev_time = {}", self.integral, self.ki, self.time, self.time_prev);
//println!("d = {}, kd = {}", d, self.kd);
//println!("Out valve = {}", self.out_valve);
//println!("\n -----{}-----", i);


//------------------------------ Run ------------------------------
//println!("\n -----{}-----", i);
//println!("In Flow is {:.2} & Out Flow is {:.2}", in_flow, out_flow);
//println!("Level is {:.2} & Setpoint is {:.2}", self.level, self.setpoint);    
//println!("Out Valve is {:.2}", self.out_valve);
//println!(" -----{}-----\n", i);


//------------------------------ TRAIN ------------------------------
/*
println!("\n -----{}-----", j);
println!("In Flow is {:.2} & Out Flow is {:.2}", in_flow, out_flow);
println!("Level is {:.2} & Setpoint is {:.2}", self.level, self.setpoint);    
println!("Out Valve is {:.2}", self.out_valve);
println!(" -----{}-----\n", j);
*/