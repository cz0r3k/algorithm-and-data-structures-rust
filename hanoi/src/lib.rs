use std::collections::VecDeque;

pub struct Towers {
    src: VecDeque<u32>,
    tmp: VecDeque<u32>,
    dst: VecDeque<u32>,
    size: usize,
}

impl Towers {
    pub fn new(size: usize) -> Self {
        let mut src: VecDeque<u32> = VecDeque::new();
        for i in (1..=size).rev() {
            src.push_back(i as u32);
        }
        Towers {
            size,
            src,
            tmp: VecDeque::new(),
            dst: VecDeque::new(),
        }
    }

    fn r#move(from: &mut VecDeque<u32>, to: &mut VecDeque<u32>) {
        to.push_back(from.pop_back().unwrap());
    }

    fn hanoi(
        size: usize,
        src: &mut VecDeque<u32>,
        dst: &mut VecDeque<u32>,
        tmp: &mut VecDeque<u32>,
    ) {
        if size > 0 {
            Towers::hanoi(size - 1, src, tmp, dst);
            Towers::r#move(src, dst);
            Towers::hanoi(size - 1, tmp, dst, src);
        }
    }

    fn print_stack(mut s: VecDeque<u32>) {
        if s.is_empty() {
            print!("empty");
        } else {
            let mut tmp_stack: VecDeque<u32> = VecDeque::new();
            while !s.is_empty() {
                Towers::r#move(&mut s, &mut tmp_stack);
            }
            while !tmp_stack.is_empty() {
                Towers::r#move(&mut tmp_stack, &mut s);
                print!("{} ", s.back().unwrap());
            }
        }
    }

    pub fn print(&self) {
        print!("Source stack:\t\t\t");
        Towers::print_stack(self.src.clone());
        println!();
        print!("Temporary stack:\t\t");
        Towers::print_stack(self.tmp.clone());
        println!();
        print!("Destination stack:\t\t");
        Towers::print_stack(self.dst.clone());
        println!("\n");
    }

    pub fn solve(&mut self) {
        Towers::hanoi(self.size, &mut self.src, &mut self.dst, &mut self.tmp);
    }
}
