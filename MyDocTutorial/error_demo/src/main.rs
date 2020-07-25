mod propagating_errors_demo;
mod result_panic;
mod vec_panic;

fn main() {
    // panic!("Crash and Burn");
    //.  vec_panic::run();
    //result_panic::run();
    propagating_errors_demo::read_username_from_file();
}
