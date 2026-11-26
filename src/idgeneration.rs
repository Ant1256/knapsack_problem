pub struct IdGenerator {
    current_id: usize,
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator {
            current_id: 0,
        }
    }

    pub fn next(&mut self) -> usize {
        let res = self.current_id;
        self.current_id += 1;
        res
    }
}
