pub fn run() {
  let numbers: [i32; 5] = [1,2,3,4,5];
  println!("{:?}", numbers);

  println!("Length of array: {}", numbers.len());
  println!("Array size in bytes {}", std::mem::size_of_val(&numbers));

  // get slice
  let arr_slice: &[i32]= &numbers;
  let slice: &[i32]= &numbers[0..2];
  println!("Slice:  {:?}", arr_slice);

  // vectors
  let mut new_vector: Vec<i32> = vec![1,2,3];

  new_vector.push(32);

  println!("The vector {:?}", new_vector);
}