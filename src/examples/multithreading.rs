use std::time::SystemTime;
use std::thread;
use std::time::Duration;

pub fn mt_test() {
    struct Total {
        total: f64,
        count: u64
    }
    impl Total {
        fn average(&self) -> f64 {
            self.total / self.count as f64
        }
        fn add_next(&mut self, next: f64) {
            let new_total = self.total + next;
            let new_count = self.count + 1;
        
            self.total = new_total;
            self.count = new_count;
        }
    }
    let mut tot = Total {
        total: 0.0,
        count: 0
    };
    let now = SystemTime::now();
    for _ in 1..10000000 {
        tot.add_next(1.0);
    }
    let mut elapsed_time = now.elapsed().unwrap().as_secs() as f64;
    let fraction = now.elapsed().unwrap().subsec_nanos() as f64;
    elapsed_time = elapsed_time + fraction * 1e-9;
    println!("\nTIME ELAPSED {}\n", elapsed_time);

    let child_thread = thread::spawn(move || {
        for _ in 0..5 {
            println!("Hello world from a different thread");
            thread::sleep(Duration::from_millis(500));
        }
    });

    for _ in 0..5 {
        println!("Hello world from the parent thread");
        thread::sleep(Duration::from_secs(1));
    }
    child_thread.join();

}

#[cfg(test)]
mod multithreading_test {
    use super::*;

    #[test]
    fn run_mt_test() {
        mt_test();
    }
}