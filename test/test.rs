fn main() {
  println!("Hello, world!");
}

fn very_safe_function() {
  println!("This is a very safe function");
}

fn very_unsafe_function() {
  println!("This is a very unsafe function");
  unsafe {
      println!("This is an unsafe block");
  }
}