mod hello_rust;
pub use self::hello_rust::print_hello;

pub fn lib_hello(){
  println!("lib_hello");
} 