use std::collections::BTreeMap;
use super::shared::*;

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, (mut moves, nodes)) = my_parser(input).unwrap();
    let mut map: BTreeMap<String, Node> = BTreeMap::new();
    for node in nodes.into_iter() {
        map.entry(node.name.clone()).or_insert(node);
    }
    let mut number_of_moves: usize = 0;
    let mut current_node_name: &String = &"AAA".to_string();
    while current_node_name != "ZZZ" {
        let current_node = map.get(current_node_name).expect("well defined input");
        let movement = moves.next().expect("should be infinite");
        match movement {
            Movement::Left => {
                current_node_name = &current_node.left;
            }
            Movement::Right => {
                current_node_name = &current_node.right;
            }
        };
        number_of_moves += 1
    }
    Ok(number_of_moves.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moveset() {
        let mut ms = MoveSet::new(vec![Movement::Right, Movement::Left, Movement::Left]);
        assert_eq!(Some(Movement::Right), ms.next());
        assert_eq!(Some(Movement::Left), ms.next());
        assert_eq!(Some(Movement::Left), ms.next());
        assert_eq!(Some(Movement::Right), ms.next());
        assert_eq!(Some(Movement::Left), ms.next());
    }

    #[test]
    fn test_case1() -> anyhow::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_case2() -> anyhow::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
