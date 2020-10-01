#[derive(Debug)]
pub enum MapValueType<'buf> {

    Single(&'buf str), 
    Multiple(Vec<&'buf str>)
}