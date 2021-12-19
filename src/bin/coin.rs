use itertools::Itertools;

fn func(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    a + b * c.pow(2) + d.pow(3) - e
}
fn main() {
    for perm in [2, 3, 5, 7, 9].iter().permutations(5) {
        if func(*perm[0], *perm[1], *perm[2], *perm[3], *perm[4]) == 399 {
            println!("{:?}", perm);
        }
    }
}
