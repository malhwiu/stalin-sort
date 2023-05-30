// Copyright 2023 Nikolai Serafimovich
// See the LICENSE file at the top-level directory of this distribution

use num::traits::Num;

pub fn stalin_sort<T: Num + std::cmp::PartialOrd>(
    mut array: Vec<T>,
    remove_repeating: bool,
) -> Vec<T> {
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
        let mut arr = vec![1, 16, 6, 8, 5, 18, 2];
        arr = stalin_sort(arr, false);
        assert_eq!(arr, vec![1, 16, 18]);

        let mut arr = vec![1, 6, 6, 8, 4, 3, 9];
        arr = stalin_sort(arr, true);
        assert_eq!(arr, vec![1, 6, 8, 9]);
    }
}
