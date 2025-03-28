/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn quick_sort<T: PartialOrd>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }

    let pviot = partition(vec);
    quick_sort(&mut vec[0..pviot]);
    quick_sort(&mut vec[pviot + 1..]);
}

fn partition<T: PartialOrd>(vec: &mut [T]) -> usize {
    let len = vec.len();
    let pivot = len / 2;

    vec.swap(pivot, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if vec[j] <= vec[len - 1] {
            vec.swap(i, j);
            i += 1;
        }
    }

    vec.swap(i, len - 1);
    i
}

fn sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    quick_sort(array);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
