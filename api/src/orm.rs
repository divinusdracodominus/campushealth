pub trait Queryable {
    fn insert(&self);
    fn select() -> Self;
    fn update(&self);
    fn delete();
}