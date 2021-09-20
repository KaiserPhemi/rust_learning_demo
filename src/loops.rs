pub fn run(){
  // infinite loop
  let mut count = 0;
  loop{
    count +=1;
    println!("Number {}", count);
    if count==60 {
      break;
    }
  }

  // while loop
  while count<=100 {
    if count % 15 == 0 {
      println!("FizzBuzz");
    }else if count % 5==0 {
      println!("Buzz");
    } else if count % 3 ==0{
      println!("Fizz");
    } else {
      println!("Count: {}", count)
    }
    count +=1;
  }

  // for loop
  for x in 0..20 {
    println!("Rust to the world {}", x);
  }

}