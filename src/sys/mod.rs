pub mod sys {
    use std::thread::JoinHandle;

    pub struct sys {
        active: bool,
        vec: Vec<Option<JoinHandle<()>>>,
    }

    impl sys {
        // 关联函数
        pub fn init() -> sys {
            let mut sys = sys {
                active: false,
                vec: Vec::new(),
            };
            sys.active = true;
            sys
        }
        pub fn print(s: String) {
            println!("{}", s)
        }
        pub fn choose(a: String, b: String) -> String {
            use tinyrand::{Rand, StdRand};
            let mut rand = StdRand::default();
            let index = rand.next_usize() % 2;
            if index == 0 {
                a
            } else {
                b
            }
        }
        pub fn stop(mut self) {
            self.active = false;
            for i in self.vec.iter_mut() {
                if let Some(handle) = i.take() {
                    handle.join().unwrap();
                }
            }
        }
        // 方法函数
        pub fn spawn(&mut self, f: fn() -> ()) {
            use std::thread;
            let handle = thread::spawn(move || {
                f();
            });
            self.vec.push(Some(handle));
            ()
        }
        // 方法函数
        pub fn sched() {
            use std::thread;
            use std::time::Duration;
            thread::sleep(Duration::from_secs(1));
        }
    }
}
