pub fn swap_and_move<T: Default>(val: &mut T) -> T {
  std::mem::replace(val, Default::default())
}

pub enum SingleOrList<T> {
  Data(T),
  List(Vec<T>),
}