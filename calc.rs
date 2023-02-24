use std::io;



trait ComputeFns{
    fn addition(&self)->f64;
    fn subtraction(&self)->f64;
    fn multiplication(&self)->f64;
    fn division(&self)->f64;
    fn reminder(&self)->f64;
}


struct Compute{
    x: f64,
    y: f64,
}

impl ComputeFns for Compute{
    fn addition(&self)->f64{
        self.x + self.y
    }
    fn subtraction(&self)->f64{
        self.x - self.y
    }
    fn multiplication(&self)->f64{
        self.x * self.y
    }
    fn division(&self)->f64{
        if self.y == 0.0{
            f64::NAN
        }
        else{
            self.x / self.y
        }
    }
    fn reminder(&self)->f64{
        if self.y == 0.0{
            f64::NAN
        }
        else{
            self.x % self.y
        }
    }

}

fn read_input(){
    let mut input_string: String = String::new();
    let mut inputs: Vec<f64> = Vec::new();


    loop{
        println!("Please type exactly two numbers (format: X Y):");
        match io::stdin().read_line(&mut input_string) {
            Ok(n)=>{
                inputs=input_string.trim().split(" ")
                .map(|x| x.parse().expect("NUT A NUMBER!")).collect();

                match inputs.len(){
                    2=>{
                        println!("You wrote {:?}", inputs,);
                        let example = Compute { x:inputs[0],
                                                y:inputs[1]};
                        println!("Addition: {}\nSubtraction: {}\nMultiplication: {}\nDivision: {}\nReminder: {}", 
                                example.addition(),example.subtraction(),
                                example.multiplication(),example.division(),
                                example.reminder());
                        
                    }
                    _=>{
                        println!("STUPID");
                    }
                }
            }
            Err(error)=>{

            }
        }
    }
    

}


fn main() {
    read_input();
}