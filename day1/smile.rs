
fn ownership(){

    let s = String::from("rustacean");

    let r = borrow_string(&s);

    println!("{}", r);  // ✅ Safe: r cannot outlive s

}

fn borrow_string<'a>(input: &'a String) -> &'a String {
    input   // Borrowed reference, tied to input's lifetime
}


fn get_ref() -> &'static String {  
    let s = String::from("coderrange");
    &s                     
}

fn heap(){


    let mut boxs =vec![1,2,3];
    let varss = Box::new(5);

    let names:&str ="coderrange";
    let mut smiles=String::from("coderrange");
    
    smiles.push_str("coderrange");
    println!("{:?} {:?} ",boxs,varss);
     
}



fn sucess(mut a:&str ,b:i32){

     println!("{} {} ",a,b);
     a="venkatesh";
}


fn main(){


    ownership();

    let a:i32=5;
    println!("{}",a);
    sucess ( "str",23);

     heap();


    let mut a1:i32=6; 
    let ptr:*const i32 = &mut a1;

    unsafe
    {
            println!("{}",*ptr);
    }

    

}