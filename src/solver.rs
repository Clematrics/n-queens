pub struct Solver {
    board_size: usize,
    solutions_found: u64,
    finished: bool,
    col: Vec<bool>,
    up: Vec<bool>,
    down: Vec<bool>,
    context: Vec<(usize, usize)>,
}

pub struct Configuration {
    pub configuration: Vec<usize>,
    pub is_valid: bool,
}

impl Solver {
    pub fn new(board_size: usize) -> Self {
        let mut col = Vec::new();
        col.resize(board_size, true);
        let mut up = Vec::new();
        up.resize(2 * board_size - 1, true);
        let mut down = Vec::new();
        down.resize(2 * board_size - 1, true);

        Self {
            board_size,
            solutions_found: 0,
            finished: false,
            col,
            up,
            down,
            context: vec![(0, 0)],
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn solutions_found(&self) -> u64 {
        self.solutions_found
    }

    fn backtrack_once(&mut self) {
        self.context.pop();
        match self.context.last().copied() {
            Some((i, j)) => {
                self.col[j] = true;
                self.down[i + j] = true;
                self.up[i + self.board_size - j - 1] = true;
                let last_pos = self.context.len() - 1;
                self.context[last_pos] = (i, j + 1);
            }
            _ => (),
        }
    }

    pub fn search(&mut self) -> Option<Configuration> {
        match self.context.last().copied() {
            Some((i, mut j)) => {
                if i == self.board_size {
                    self.backtrack_once();
                    return self.search();
                }

                while j < self.board_size
                    && !(self.col[j] && self.down[i + j] && self.up[i + self.board_size - j - 1])
                {
                    j += 1;
                }

                if j == self.board_size {
                    self.backtrack_once();
                    return self.search();
                }

                let last_pos = self.context.len() - 1;
                self.context[last_pos] = (i, j);
                self.col[j] = false;
                self.down[i + j] = false;
                self.up[i + self.board_size - j - 1] = false;

                let configuration = self.context.iter().map(|(_, j)| *j).collect();

                self.context.push((i + 1, 0));
                if i + 1 == self.board_size {
                    self.solutions_found += 1;
                }
                Some(Configuration {
                    configuration,
                    is_valid: i + 1 == self.board_size,
                })
            }
            None => {
                self.finished = true;
                None
            }
        }
    }
}
