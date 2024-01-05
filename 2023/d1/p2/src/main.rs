use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut lookup_tree = RadixNode::new(None);
    lookup_tree.add_value("1", 1);
    lookup_tree.add_value("2", 2);
    lookup_tree.add_value("3", 3);
    lookup_tree.add_value("4", 4);
    lookup_tree.add_value("5", 5);
    lookup_tree.add_value("6", 6);
    lookup_tree.add_value("7", 7);
    lookup_tree.add_value("8", 8);
    lookup_tree.add_value("9", 9);
    lookup_tree.add_value("one", 1);
    lookup_tree.add_value("two", 2);
    lookup_tree.add_value("three", 3);
    lookup_tree.add_value("four", 4);
    lookup_tree.add_value("five", 5);
    lookup_tree.add_value("six", 6);
    lookup_tree.add_value("seven", 7);
    lookup_tree.add_value("eight", 8);
    lookup_tree.add_value("nine", 9);

    let mut sum = 0;
    let reader = BufReader::new(File::open("input.txt").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let d1 = (0..line.len())
            .map(|i| lookup_tree.find_first_match(&line[i..]))
            .find(|x| x.is_some())
            .flatten()
            .unwrap();
        let d2 = (0..line.len())
            .rev()
            .map(|i| lookup_tree.find_first_match(&line[i..]))
            .find(|x| x.is_some())
            .flatten()
            .unwrap();
        let num = format!("{d1}{d2}").parse::<u32>().unwrap();
        sum += num;
    }
    println!("{sum}");
}

struct RadixNode {
    children: HashMap<char, RadixNode>,
    val: Option<u32>,
}

impl RadixNode {
    fn new(val: Option<u32>) -> Self {
        Self {
            children: HashMap::new(),
            val,
        }
    }

    fn add_value(&mut self, s: &str, val: u32) {
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_insert(RadixNode::new(None));
        }
        node.val = Some(val);
    }

    fn find_first_match(&self, s: &str) -> Option<u32> {
        let mut node = self;
        for c in s.chars() {
            if let Some(val) = node.val {
                return node.val;
            } else if let Some(next) = node.children.get(&c) {
                node = next;
            } else {
                return None;
            }
        }
        return node.val;
    }
}
