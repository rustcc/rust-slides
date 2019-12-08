// A Linear Feedback Shift Register

struct Lfsr {
    start: u16,
}

impl Iterator for Lfsr {
    type Item = u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        let bit  = ((self.start >> 0) ^ 
                    (self.start >> 2) ^ 
                    (self.start >> 3) ^ 
                    (self.start >> 5)) & 1;

        self.start =  (self.start >> 1) | (bit << 15);
        Some(bit)
    }
}

fn new_lfsr(n: u16) -> Lfsr {
    Lfsr { start: n }
}


fn main() {

    let l = new_lfsr(0x1234);

    for bit in l {
        println!("{}", bit);
    }
    
}


