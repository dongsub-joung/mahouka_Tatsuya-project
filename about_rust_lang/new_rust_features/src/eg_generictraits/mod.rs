pub trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, f: F) where F: Fn(T);
}

impl dyn Seq for Seq{
    fn len(&self)-> u32{
        
    }
}