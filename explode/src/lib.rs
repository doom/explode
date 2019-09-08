
pub trait Explode {
    type Tuple;

    fn explode(self) -> Self::Tuple;
}
