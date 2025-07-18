

fn tohappy(x: &mut i32 ){
    
    *x=20;
    println!("{:p} {:p} {} main ",&x,x,*x);
}

fn sucess(x: &mut i32 ){
    
    println!("{:p} {} sucess ",&x ,x);
    tohappy( x )
    
}

fn main(){
    
 /*    
   Stack    10     to sucess   tohappy
           pass     address    20
    
     Heap   10     to sucess. tohappy
            pass     address   20 
            
   */
   
  
  let mut x:i32 =10;
  println!("{:p} {} main ",&x,x);
  sucess(&mut x);
 println!("{} end main ",x);
   
}

