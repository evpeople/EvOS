mod sys;
fn main() {
    println!("Hello, world!");
    let mut sys = sys::sys::sys::init();
    for i in "abcded".chars() {
        let name = i.to_string();
        sys.spawn(name, Tprint);
    }
    sys.stop()
}
fn Tprint(name: String) {
    let mut count: usize = 0;
    for i in 1..10 {
        count += 1;
        let s = format!("{}:{}", name.clone(), count.to_string());
        sys::sys::sys::print(s);
        sys::sys::sys::sched();
    }
}
