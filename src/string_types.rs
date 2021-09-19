pub fn run(){
  // immutable fixed lenght 
  let str_type = "Phemi";
  println!("{}", str_type);

  // muttable 
  let my_name = String::from("Oluwafemi");
  println!("Length of my name: {}", my_name.len());
  println!("Capacity of name: {}", my_name.capacity());

}