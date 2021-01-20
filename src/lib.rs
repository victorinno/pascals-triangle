pub struct PascalsTriangle {
    pub rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut pascals_triangle = PascalsTriangle { rows: vec![] };
        (0..row_count).for_each(|r| pascals_triangle.rows.push(PascalsTriangle::calc(r)));
        pascals_triangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        Vec::from(self.rows.as_slice())
    }

    fn calc(row_count: u32) -> Vec<u32> {
        let mut row: Vec<u32> = vec![1];
        for p in 1..(row_count + 1) {
            if let Some(&last) = row.last() {
                row.push((last * (row_count + 1 - p)) / p)
            }
        }
        row
    }
}
