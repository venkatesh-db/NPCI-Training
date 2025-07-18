

fn tohappy(x: &mut Box<i32> ){
    
    **x=20;
    
     let heapptr: *const i32= &**x;
     println!(" heapptr {:p} ", heapptr);
   
    
    println!("{:p} {:p} {} main ",&x,x,*x);
}

fn sucess(x: &mut Box<i32> ){

 
    
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
   
  
  let mut x:Box<i32> =Box::new(10);
  println!("{:p} {} main ",&x,x);
  let heapptr:*const i32= &*x;
   println!(" heapptr {:p} ", heapptr);
  
  //println!("{:p} box heap");
 
   sucess(&mut x);
 //println!("{} end main ",x);
   
}

