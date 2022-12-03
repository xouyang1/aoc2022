pub fn dot_product_1d_borrow_borrow<'a, Input1, Input2, Output>(a: Input1, b: Input2) -> Output
where
    Input1: Iterator<Item = &'a Output>,
    Input2: IntoIterator<Item = &'a Output>,
    Output: 'a + std::ops::Mul<Output = Output> + Copy,
    Output: std::iter::Sum<<Output>::Output>,
{
    a.zip(b).map(|(&x, &y)| x * y).sum()
}

pub fn dot_product_1d_move_borrow<'a, Input1, Input2, Output>(a: Input1, b: Input2) -> Output
where
    Input1: Iterator<Item = Output>,
    Input2: IntoIterator<Item = &'a Output>,
    Output: 'a + std::ops::Mul<Output = Output> + Copy,
    Output: std::iter::Sum<<Output>::Output>,
{
    a.zip(b).map(|(x, &y)| x * y).sum()
}

pub fn dot_product_2d<'a, Input1, Input2, Output>(a: Input1, b: Input2) -> Output
where
    Input1: Iterator,
    Input2: IntoIterator,
    <Input1 as Iterator>::Item: IntoIterator<Item = &'a Output>,
    <Input2 as IntoIterator>::Item: IntoIterator<Item = &'a Output>,
    Output: 'a + std::ops::Mul<Output = Output> + Copy,
    Output: std::iter::Sum<Output>,
{
    a.zip(b)
        .map(|(x, y)| dot_product_1d_borrow_borrow(x.into_iter(), y))
        .sum()
}
