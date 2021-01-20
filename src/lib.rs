pub struct PascalsTriangle {
    pub rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return PascalsTriangle { rows: Vec::new() };
        }

        let mut pascals_triangle = PascalsTriangle { rows: vec![] };

        for i in 0..row_count {
            if i == 0 {
                pascals_triangle.rows.push(vec![1]);
            } else {
                let mut row = Vec::new();
                let index: usize = (i - 1) as usize;
                let before: &Vec<u32> = pascals_triangle.rows.get(index).expect("expected to have at least frist row");
                for sum in 0..(before.len() + 1) {
                    if sum == 0 || sum == before.len() {
                        row.push(1);
                    } else {
                        row.push(before.get(sum).expect("expects A") + before.get(sum - 1).expect("expects B"));
                    }
                }
                pascals_triangle.rows.push(row);
            }
        }

        pascals_triangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        Vec::from(self.rows.as_slice())
    }
}
