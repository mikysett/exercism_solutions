pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return Self(vec![]);
        }
        let mut triangle = vec![vec![1]];
        for row_i in 1..row_count as usize {
            let mut row = vec![1];
            for i in 0..row_i - 1 {
                row.push(triangle[row_i - 1][i] + triangle[row_i - 1][i + 1]);
            }
            row.push(1);
            triangle.push(row);
        }
        Self(triangle)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
