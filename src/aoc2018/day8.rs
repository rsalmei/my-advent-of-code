use crate::Input;

pub fn run(input: Input) {
    let data = input.text();

    // part one.
    let tree = data
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Node>();
    let sum = tree.preorder_fold(0, &mut |acc, node| acc + node.sum_meta());
    println!("{sum}");

    // part two.
    println!("{}", tree.value());
}

struct Node {
    children: Vec<Node>,
    meta: Vec<u32>,
}

impl Node {
    fn preorder_fold<T>(&self, acc: T, f: &mut impl Fn(T, &Node) -> T) -> T {
        let acc = f(acc, self);
        self.children
            .iter()
            .fold(acc, |acc, child| child.preorder_fold(acc, f))
    }
    fn value(&self) -> u32 {
        if self.children.is_empty() {
            return self.sum_meta();
        }
        self.meta
            .iter()
            .map(|&n| {
                self.children
                    .get(n as usize - 1)
                    .map(|child| child.value())
                    .unwrap_or_default()
            })
            .sum()
    }
    fn sum_meta(&self) -> u32 {
        self.meta.iter().copied().sum()
    }
}

impl FromIterator<u32> for Node {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        from_iter(&mut it)
    }
}
fn from_iter(it: &mut impl Iterator<Item = u32>) -> Node {
    let mut children = Vec::with_capacity(it.next().unwrap() as usize);
    let mut meta = Vec::with_capacity(it.next().unwrap() as usize);
    for _ in 0..children.capacity() {
        children.push(from_iter(it))
    }
    for _ in 0..meta.capacity() {
        meta.push(it.next().unwrap())
    }
    Node { children, meta }
}
