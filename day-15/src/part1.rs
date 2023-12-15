use super::shared::my_hash;

pub fn process(input: &str) -> u32 {
    input.split(',').map(my_hash).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, process(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!(518107, process(input));
    }
}
