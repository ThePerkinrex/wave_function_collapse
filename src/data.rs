#[derive(Debug, Clone, PartialEq)]
pub enum Data {
	Error,
    Collapsed(usize),
    Options(Vec<usize>),
}

impl Data {

}
