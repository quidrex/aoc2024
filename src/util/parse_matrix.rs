use ndarray::Array2;

pub trait ToMatrix<T, F>
where
    F: Fn(char) -> T,
{
    fn parse_matrix(&self, f: F) -> Array2<T>;
}

impl<T, F> ToMatrix<T, F> for &str
where
    F: Fn(char) -> T,
{
    fn parse_matrix(&self, f: F) -> Array2<T> {
        let col_count = self.find("\n").unwrap();
        let row_count = self.len() / (col_count + 1);

        let vec = self
            .chars()
            .filter_map(|c| if c == '\n' { None } else { Some(f(c)) })
            .collect::<Vec<T>>();
        Array2::from_shape_vec((row_count, col_count), vec).unwrap()
    }
}
