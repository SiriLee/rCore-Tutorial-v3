// A conventional Rust program (with std) built for a real Linux target.
// Shows what the libc startup path demands from the kernel.
//
// Build: run project/scripts/build-programs.sh
fn main() {
    println!("[B] hello from std + musl");
    let args: Vec<String> = std::env::args().collect();
    println!("[B] argv = {:?}", args);
}
