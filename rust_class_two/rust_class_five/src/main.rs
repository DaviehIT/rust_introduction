fn main() {
    
    //annotate
    //let a:i32=3;
   // let b:i32=a;
   // println!("number {a}");


    //string
    //str
    //let c:String= String::from(s:"Hello");
    //hello(c);
   // let b:String=c;
   // print!("number {c} -- {b}");
   //fn hello(param_1:String)->String {
  //  return param_1;
   

  // let s1:String=String::from(s:"hello");
  // let len:usize=calculate_length(&s1);
 //  println!("The length of '{} is {}",s1,len);

 //let a String=String::from("Hello");
//let b=sample(a);
//println!("{}{}",a,b);

let mut a:String="hello".to_string();
let b:$String=insert_to_db(&a);
println!("Hello b= {b}");

fn insert_to_db(par:&mut String)->&String{
    //code for inserting db
    par.push_str(String:"World");
    return par;
}
}

//fn sample(s:String);{
    //return s;
//}



