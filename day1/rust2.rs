
fn pass(rets:&str) -> &str{

    return rets ;
}

fn news<'a>(args: &'a String) -> &'a String { args }


fn main() {
 
 let rets= String::from("coderrange");

  let rets1:&str=   pass(&rets);

  println!("{}",rets1);

  news(&rets);

  let a=23;

  match a{
    1=> println!("hello smiler"),
    _=>println!("hello smiler"),
  }

}


