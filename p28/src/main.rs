extern "C" {
    fn run_main();
}

fn main() {
    unsafe { run_main() };
}
