/* select the cols and rows */
use std::io::Stdin;
pub struct TableSelector {
    rows: usize,
    cols: Vec<usize>,
}
pub struct Builder {
    rows: usize,
    cols: Vec<usize>,
}
impl Builder {
    pub fn set_choose_cols(mut self, cols: Vec<usize>) -> Self {
        self.cols = cols;
        self
    }
    pub fn set_head_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }
    pub fn build(self) -> TableSelector {
        TableSelector {
            rows: self.rows,
            cols: self.cols,
        }
    }
}
impl TableSelector {
    pub fn builder() -> Builder {
        Builder {
            rows: 0,
            cols: vec![],
        }
    }
    pub fn tb_select(&self, in_flow: Stdin) {
        let lines = in_flow.lines().map(|l| l.unwrap());
        for (linenum, line) in lines.enumerate() {
            if linenum + 1 <= self.rows {
                continue;
            }
            let words: Vec<&str> = line.split(' ').collect();
            for col_num in self.cols.iter() {
                if *col_num <= words.len() - 1 {
                    print!("{} ", words[*col_num]);
                } else {
                    print!("{} ", " ");
                }
            }
        }
    }
}
