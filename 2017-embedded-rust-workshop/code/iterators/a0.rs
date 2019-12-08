
struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            Some(r)
        }
    }
}

fn main() {
    let r = Range { start: 1, end: 10 };

    for x in r {
        println!("{}", x);  
    }
}
            
