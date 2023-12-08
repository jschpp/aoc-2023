use super::shared::*;
use std::collections::BTreeMap;

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, (mut moves, nodes)) = my_parser(input).unwrap();

    let mut map: BTreeMap<String, Node> = BTreeMap::new();
    for node in nodes.clone().into_iter() {
        map.entry(node.name.clone()).or_insert(node);
    }

    let end_node_count = nodes
        .iter()
        .filter(|node| node.name.ends_with('Z'))
        .count();
    let mut current_nodes: Vec<Node> = nodes
        .into_iter()
        .filter(|node| node.name.ends_with('A'))
        .collect();
    let start_node_count = current_nodes.len();

    // sanity checking since I was playing around with the input
    assert_eq!(start_node_count, end_node_count);

    let mut cycle_end: Vec<usize> = Vec::new();
    let mut number_of_moves: usize = 0;
    while !current_nodes.is_empty() {
        let mut tmp: Vec<Node> = Vec::new();
        let movement = moves.next().expect("should be infinite");
        for node in current_nodes.iter() {
            let next_node = match movement {
                Movement::Left => map.get(&node.left),
                Movement::Right => map.get(&node.right),
            }
            .expect("well defined input");

            // this only works because the input is defined the way
            // that the circle begins with the first occurrence of an node with a Z at the end
            if next_node.name.ends_with('Z') {
                cycle_end.push(number_of_moves + 1);
            } else {
                tmp.push(next_node.clone());
            }
        }
        number_of_moves += 1;
        current_nodes = tmp;
    }
    let number_of_moves = vec_lcm(cycle_end);
    Ok(number_of_moves.to_string())
}

fn vec_lcm(input: Vec<usize>) -> usize {
    input.iter().fold(1, |acc, x| lcm(acc, *x))
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        let t = a;
        a = b;
        b = t % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() -> anyhow::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
