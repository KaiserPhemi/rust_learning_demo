pub fn run(){

  // immutable
  let name ="Oluwafemi Akinwa";

  // muttable variable
  let mut age = 0;  
  println!("{} will be {} on 30th",name, age);
  age = 1;
  println!("{} will be {} on 30th",name, age);

  // const variable
  const PHONE: i64 = 23469636303;
  println!("phone number{}",PHONE);

  // assigning multiple variables
  let (user_name, new_age) = ("Phemi", 1);
  println!("{} is {}", user_name, new_age);
}