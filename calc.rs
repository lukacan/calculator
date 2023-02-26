use std::io;
mod secret{
    trait ComputeFns{
        fn addition(&self)->f64;
        fn subtraction(&self)->f64;
        fn multiplication(&self)->f64;
        fn division(&self)->f64;
        fn modulo(&self)->f64;
    }
    pub struct Compute{
        x: f64,
        y: f64,
    }
    impl Compute{
        pub fn new(arg1:&f64,arg2:&f64)->Self
        {
            Self {x:*arg1,y:*arg2}
        }

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
        fn modulo(&self)->f64{
            if self.y == 0.0{
                f64::NAN
            }
            else{
                self.x.rem_euclid(self.y)
            }
        }
    }
    pub fn print_results(example: &Compute){
        println!("Numbers entered X: {}, Y: {}",example.x,example.y);
        println!("Addition (X + Y): {}",example.addition());
        println!("Subtraction (X - Y): {}",example.subtraction());
        println!("Multiplication (X * Y): {}",example.multiplication());
        println!("Division (X / Y): {}",example.division());
        println!("Modulo (X mod Y): {}",example.modulo()); 
    }
}

fn read_num(read_into:&mut f64){
    let mut input_string: String = String::new();
    loop{
        println!("Enter number:");
        input_string.clear();
        match io::stdin().read_line(&mut input_string){
            Ok(_n) =>{
                match input_string.trim().parse::<f64>(){
                    Ok(f)=>{
                        // change here
                        *read_into = f;
                    }
                    _=>{
                        continue;        
                    }
                       
                }
                break;    
            }
            _=>{
                continue;
            }   
        }
    }
}
fn calc(){
    // declare here
    let mut x_in: f64 = 0.0;
    let mut y_in: f64 = 0.0;

    // pass here as mutable reference,
    // needs to be mutable in order to change it
    // so basically we not changing ownership, just borrowing and telling
    // the varaible can be changed
    read_num(&mut x_in);
    read_num(&mut y_in);


    // call public constructor, x_in and y_in are borrowed only
    // without mutability
    let example = secret::Compute::new(&x_in,&y_in);
    
    // won`t work, no visible
    //println!("Addition (X + Y): {}",example.addition());

    // pass as immutable ref, so borrow only without rights to change
    secret::print_results(&example);
}

fn main() {
    calc();
}