// mormal struct
// struct Color {
//     red:u8,
//     green: u8,
//     blue: u8,
//   }

// tuple struct
// struct Color(u8, u8, u8);

struct Person  {
  f_name: String,
  l_name: String
}

impl Person {
  fn new(first:&str, last:&str) -> Person {
    Person {
      f_name: first.to_string(),
      l_name: last.to_string()
    }
  }
}

pub fn run(){
//  let mut col = Color {
//    red: 255,
//    green: 80,
//    blue: 0
//  };
// let mut col = Color(244, 0, 0);
//  col.1 = 100;
// println!("Color {}, {}, {}", col.0, col.1, col.2);

let mut new_person = Person::new("Phemi", "Akinwa");

println!("Person {}, {}", new_person.f_name, new_person.l_name);

}