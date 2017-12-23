pub fn knot_hash_partial(input: &[usize], list_size: usize, rounds: usize) -> Vec<usize> {
    let mut list: Vec<_> = (0..list_size).collect();
    let mut start = 0;
    let mut step = 0;

    for _ in 0..rounds {
        for n in input.iter() {
            for i in 0..n / 2 {
                list.swap((start + i) % list_size, (start + n - i - 1) % list_size);
            }
            start += n + step;
            step += 1;
        }
    }

    list
}

pub fn knot_hash(input: &str, list_size: usize) -> String {
    let input: Vec<_> = input
        .bytes()
        .map(usize::from)
        .chain(vec![17, 31, 73, 47, 23].into_iter())
        .collect();

    knot_hash_partial(&input, list_size, 64)
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, n| acc ^ n))
        .map(|n| format!("{:02x}", n))
        .collect()
}
