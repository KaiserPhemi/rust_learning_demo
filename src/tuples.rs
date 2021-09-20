

pub fn run(){
// tuples are values of different types grouped together

let person: (&str, &str, i16) = ("Oluwafemi", "Ak", 4);
println!("{} is my name, {} is my other name and I'm {}", 
person.0, 
person.1, 
person.2);

}