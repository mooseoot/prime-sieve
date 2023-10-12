struct PrimeSieve {
    numbers: Vec<bool>,
    primes: Vec<usize>,
    start: usize,
    cursor: usize,
}

fn new_sieve(capacity: usize) -> PrimeSieve {
    PrimeSieve {
        numbers: vec![false; capacity],
        primes: Vec::new(),
        start: 2,
        cursor: 0,
    }
}

impl PrimeSieve {
    fn sieve(&mut self, prime: usize) {
        let first_mult = {
            match self.start % prime {
                 0 => 0,
                 n => prime - n,
            }
        };
        if first_mult < self.numbers.len() {
            for i in (first_mult.. self.numbers.len()).step_by(prime) {
                self.numbers[i] = true;
            }
        } 
    }
}

impl Iterator for PrimeSieve {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.primes.last() {
            Some(p) => {self.sieve(*p);},
            None => {},
        }

        let mut i = self.cursor; 
        loop {
            if i == self.numbers.len() {
                self.start += self.numbers.len(); 
                for b in &mut self.numbers {
                    *b = false;
                }
                for p in &self.primes.clone() {
                    self.sieve(*p);    
                }
                i = 0;
            } else {
                if self.numbers[i] == false {
                    break; 
                }
                i += 1;
            }
        }
        let new_p = self.start + i;
        self.primes.push(new_p);
        self.cursor = i + 1;
        return Some(new_p);
    }
}

const GB : usize = 1000000000;

fn main() {
    
    // Computes the number of primes less than n,
    // then gives the x/ln(x) approximation
    // The absolute error trends to zero by the PNT

    let n = 100000000;
    let mut m = 0;
    let mut s = new_sieve(GB/10); 
    while s.next().unwrap() <= n {
        m += 1;
    }
    println!("pi({}) = {}", n, m);
    println!("Approximation: {}", (n as f64)/f64::ln(n as f64))
}
