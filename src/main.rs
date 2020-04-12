use mesax::parser::*;

fn main() {
    println!("Hello, World!!");
    println!("Parsing is {:?}", whole_expr("sum(1 + 41, 4 + 2 * 4)"));
}

