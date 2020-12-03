#[derive(Debug, Eq, PartialEq)]
enum Error {
    SwapError,
}

fn swap<T>(a: usize, b: usize, container: Vec<T>) -> Result<Vec<T>, Error>
where
    T: Clone + std::fmt::Debug,
{
    let from = std::cmp::min(a, b);
    let to = std::cmp::max(a, b);

    let swapped = container
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<T>, (i, x)| {
            let val = if i  == from {
                container.get(to)
            } else if i == to {
                 container.get(from)
            } else {
                Some(x)
            }.unwrap();
            acc.push(val.to_owned());
            acc
        });
    Ok(swapped)
}

fn find_min_value_index<T>(i: usize, v: &Vec<T>) -> usize
where
    T: PartialOrd
{
    let mut min_index = i;
    for (pos, val) in  v[i..].iter().enumerate() {
        if val < &v[min_index] {
            min_index = pos + i;
        }
    }
    min_index
}

pub fn selection_sort<T>(src: &Vec<T>) -> Vec<T>
where
    T: Clone + std::fmt::Debug + PartialOrd + Copy,
{
    let mut cloned = src.clone();
    (0..cloned.len())
        .fold(Vec::new(), |mut acc: Vec<T>, i| {
            let min_index = find_min_value_index(i, &cloned);
            let min_value = cloned[min_index];
            acc.push(min_value);
            cloned = swap(i, min_index, cloned.clone()).unwrap();
            acc
        })
}

#[cfg(test)]
mod test {
    use super::{selection_sort, swap, find_min_value_index};

    #[test]
    fn it() {
        let data = vec![8, 4, 3, 7, 6, 5, 2, 1];
        let original = data.clone();
        assert_eq!(selection_sort(&data), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(&data, &original);

        let data = vec![1, 2, 3, 4, 5, 6, 7,  8];
        let original = data.clone();
        assert_eq!(selection_sort(&data), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(&data, &original);

        let data = vec![4, 5, 4, 6, 2, 1, 3];
        let original = data.clone();
        assert_eq!(selection_sort(&data), vec![1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(&data, &original);
    }

    #[test]
    fn swap_spec() {
        let v = vec![1, 2, 3];
        let result = swap(0, 2, v);
        assert_eq!(result, Ok(vec![3, 2, 1]));
    }

    #[test]
    fn find_min_value_index_spec() {
        let v = vec![3, 1, 2];
        let result = find_min_value_index(0, &v);
        assert_eq!(result, 1);
    }
}