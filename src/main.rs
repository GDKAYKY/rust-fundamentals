mod binary_search;
mod control_flow;
mod generate_array;
mod hashmap;
mod loops;
mod structs;
pub mod return_values;
mod variable_assignment;
pub mod variable_shadowing;
pub mod variable_ownership;
fn main() {
    println!("tests");
    binary_search::binary_search();
    loops::while_loop();
}