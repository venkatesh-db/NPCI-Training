
static p1: i8 =23;

fn datatypes (){

     let p2: i8 =23;
     Box::new(5i8);
     let p3:f32=3.14;
     
     let mut refs1= Vec::new();
     refs1.push("venkatesh");
    
  /*   
  i16
 i32 
 u32
 f32
 f64
 */
 let args:bool= true;
 
}





fn main(){

let mut name:&str="venkatesh";
name="bond";
//name[0]='v';

let mut owners = String::from("heap memory");
let mut owners1=owners;
println!("{}",name);
//println!("{}",owners1);
owners1.push_str("add");
println!("{}",owners1);
}




fn owner(boxer:Box<i32> ){
    println!(" Owner changed");  
}


fn multiple(boxer:& Box<i32>){
    let copy=boxer.clone();
    owner(copy );
}


fn manybowrrower(boxer:&mut Box<i32>){

      **boxer = 7;
      
     println!(" manybowrrower modifed {}",boxer);

    multiple(& boxer);

}

fn main(){
    
    /*
    var slcie []int = make([]int , 5)
    int *ptr= new int[5]; 
     int ptr = new int[5];
 */
 
    let mut boxing= Box::new(5); 
    // boxing can be many bowrrorer 
    // boxing is only owneer 
    
    println!("inital value {}",boxing); 
    
    manybowrrower( &mut boxing);

    println!("{}",boxing);
}



fn owner(boxer:Box<i32> ){
    println!(" Owner changed");  
}


fn multiple(boxer:&Box<i32>){
    let copy=boxer.clone();
    owner(copy );
}


fn manybowrrower(boxer:&Box<i32>){
    multiple(&boxer);
    println!(" manybowrrower {}",boxer);
}

fn main(){
    
    /*
    var slcie []int = make([]int , 5)
    int *ptr= new int[5]; 
     int ptr = new int[5];
 */
 
    let boxing= Box::new(5); 
    // boxing can be many bowrrorer 
    // boxing is only owneer 
    manybowrrower( &boxing);
    println!("{}",boxing);
}



fn retruns()->&'static i32 {
    static rests:i32=99;// data segement 
    return &rests;  // ref is register 
}

fn differnce(){

  /*
            *p --> 4 byte    --> address 
            &a --> *-->  byte  ---> address
            

            p --> address  --> manulaly *
            a --> address --> fetch value
            
  */


   let x:i8=5;

   let y:&i8 = &x;
   let ptr:*const i8 = &x;
    
    println!("{:p} {:p} {}",&y,&x,y);
    unsafe{
     println!("{:p} {}", ptr,*ptr);
     }
}


fn communicate( smiles:i8 ,smilesq:&i8 ){
         println!(" communicate {} {}", smiles,smilesq);
}


fn bowrroing(){
    
     let smies:i8=7;
     communicate( smies , &smies);

      println!(" borrwroing {}", smies);
    
}





fn main( ){

     let rest:&i32;

     rest=retruns();
    
     println!(" returns {}", rest);
    
    
    differnce();
    
    bowrroing()
}
