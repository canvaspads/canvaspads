pub trait Component {
    type Output;

    fn render(&self) -> Self::Output;
}
