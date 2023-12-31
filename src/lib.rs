mod arr;
mod link;
mod string;
mod num;
mod greed;
mod dynamic;
mod math;
mod tree;
mod graph;

mod other;
pub fn add(left: usize, right: usize) -> usize {
    left + right
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
