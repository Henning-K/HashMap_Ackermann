use std::collections::HashMap;
use std::time::Instant;

struct Ack {
    values: HashMap<(usize, usize), usize>,
}

impl Ack {
    fn new() -> Self {
        Ack { values: HashMap::new() }
    }

    fn get_or_calc(&mut self, fst: usize, snd: usize) -> usize {
        if let Some(&x) = self.values.get(&(fst, snd)) {
            return x;
        }
        let val = self.ack(fst, snd);
        self.values.insert((fst, snd), val);
        val
    }

    fn ack(&mut self, m: usize, n: usize) -> usize {
        if m==0 {
            n+1
        } else if n==0 {
            self.get_or_calc(m-1, 1)
        } else {
            let snd = self.get_or_calc(m, n-1);
            self.get_or_calc(m-1, snd)
        }
    }
}

fn main() {
    let mut ack = Ack::new();

    for i in 0..5 {
        for j in 0..11 {
            let start = Instant::now();
            let val = ack.ack(i, j);
            let duration = start.elapsed();
            print!("({},{}) = {}, ", i, j, val);
            println!("Time taken: {} seconds",
                duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9);
        }
    }
}
