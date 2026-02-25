mod variable_assignment;
mod control_flow;
mod generate_array;
mod binary_search;
pub mod variable_shadowing;

fn main() {
    println!("testss");
    
    variable_assignment::main();
    control_flow::main();
    binary_search::binary_search();
}