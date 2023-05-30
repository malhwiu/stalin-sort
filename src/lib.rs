use std::ops::Add;

pub fn stalin_sort<T: Add<Output = T> + std::cmp::PartialOrd>(
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
    fn t_i32() {
        let mut arr = vec![1, 16, 6, 8, 5, 18, 2];
        arr = stalin_sort(arr, false);
        assert_eq!(arr, vec![1, 16, 18]);
    }

    #[test]
    fn t_i32d() {
        let mut arr = vec![1, 6, 6, 8, 4, 3, 9];
        arr = stalin_sort(arr, true);
        assert_eq!(arr, vec![1, 6, 8, 9]);
    }
     #[test]
    fn t_u8() {
        let expected: Vec<u8> = vec![1, 6, 6, 8, 9];
        let mut arr:Vec<u8> = vec![1, 6, 6, 8, 4, 3, 9];
        arr = stalin_sort(arr, false);
        assert_eq!(arr, expected);
    }

    #[test]
    fn t_u8d() {
        let expected: Vec<u8> = vec![1, 6, 8, 9];
        let mut arr:Vec<u8> = vec![1, 6, 6, 8, 4, 3, 9];
        arr = stalin_sort(arr, true);
        assert_eq!(arr, expected);
    }
    #[test]
    fn t_f32() {
        let expected: Vec<f32> = vec![-3.2, 8.7, 9.6];
        let mut arr:Vec<f32> = vec![-3.2, -4.2, 8.7, 9.6, -4.5];
        arr = stalin_sort(arr, false);
        assert_eq!(arr, expected);
    }

    #[test]
    fn t_f32d() {
        let expected: Vec<f32> = vec![-3.2, 8.7, 9.6];
        let mut arr:Vec<f32> = vec![-3.2, -4.2, 8.7, 8.7, 9.6, -4.5];
        arr = stalin_sort(arr, true);
        assert_eq!(arr, expected);
    }
}
