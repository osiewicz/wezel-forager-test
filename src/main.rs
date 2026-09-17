mod workload;

fn main() {
    println!("Hello, world!");
    println!("Expanded workload: {}", workload::expanded_workload(42));
    println!(
        "Second expanded workload: {}",
        workload::second_expanded_workload(42)
    );
}
// trigger
