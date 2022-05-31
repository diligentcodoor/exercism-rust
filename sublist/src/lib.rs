#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (a, b) if a == b => {
            if _first_list == _second_list {
                return Comparison::Equal;
            } else {
                return Comparison::Unequal;
            }
        }
        (a, b) if a > b => {
            for window in _first_list.windows(b) {
                if window == _second_list {
                    return Comparison::Superlist;
                }
            }
            return Comparison::Unequal;
        }
        (a, b) if a < b => {
            for window in _second_list.windows(a) {
                if window == _first_list {
                    return Comparison::Sublist;
                }
            }
            return Comparison::Unequal;
        }
        (_, _) => unreachable!(),
    }
}
