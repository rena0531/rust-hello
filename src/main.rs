mod hello_rust;

pub fn print_hello() -> (){
    println!("Hello World");
  } 

fn main() {
    hello_rust::print_hello();     
    print_hello();
}
