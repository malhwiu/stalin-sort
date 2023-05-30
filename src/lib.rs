use num::traits::Num;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn stalin_sort<N: std::cmp::PartialOrd>(mut array: Vec<N>, remove_repeating: bool) -> Vec<N> {
    let mut length: usize = array.len();
    let mut current_pos: usize = 0;

    while (current_pos + 1) < length {
        current_pos += 1;

        if (array[current_pos - 1] > array[current_pos])
            || (remove_repeating & (array[current_pos - 1] >= array[current_pos]))
        {
            array.remove(current_pos);
            current_pos -= 1;
            length = array.len();
        }
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
