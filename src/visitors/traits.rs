pub trait Visit<T> {
    fn visit(&mut self, instruments: &mut Vec<T>);
    fn par_visit(&mut self, instruments: &mut Vec<T>);
}
