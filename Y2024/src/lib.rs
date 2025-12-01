#![allow(non_snake_case)]

use std::path::Path;
use std::{fs, str::FromStr};

mod macros;

pub fn read_file_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let canonical_path = fs::canonicalize(path).expect("Path {path:?} is invalid");
    println!("Reading file: {canonical_path:?}");

    fs::read_to_string(canonical_path)
        .expect("Can't read file: {path:?}")
        .lines()
        .map(String::from)
        .collect()
}

// --------------------- 5_print_queue.rs ---------------------
pub fn load_sections_from_file(path: &str) -> (Vec<String>, Vec<String>) {
    // load the file content with empty line as a divider between TWO sections
    let mut content = read_file_lines(path);

    let (divider_idx, _) = content
        .iter()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .expect("empty line not found");

    let mut section_two: Vec<String> = content.split_off(divider_idx);
    section_two.remove(0);

    (content, section_two)
}

pub fn parse_struct_from_str<F: FromStr>(strings: Vec<&str>) -> Result<Vec<F>, F::Err> {
    // split s and parse every string to F, then collect and return
    strings
        .iter()
        .map(|s| s.parse::<F>())
        .collect::<Result<Vec<F>, F::Err>>()
}

// --------------------- TREE-LIB ------------------------------
use std::mem;

#[derive(Debug)]
enum Node<Item> {
    Leaf(Item),
    Children(Vec<Node<Item>>),
}

impl<It> Node<It> {
    fn traverse(&self, f: impl Fn(&It)) {
        match self {
            Node::Leaf(item) => f(item),
            Node::Children(children) => {
                children.iter().for_each(|n| n.traverse(&f));
            }
        }
    }

    fn iter(&self) -> NodeIter<'_, It> {
        NodeIter {
            child: std::slice::from_ref(self),
            parent: None,
        }
    }
}

struct NodeIter<'a, It> {
    child: &'a [Node<It>],
    parent: Option<Box<NodeIter<'a, It>>>,
}

impl<It> Default for NodeIter<'_, It> {
    fn default() -> Self {
        NodeIter {
            child: &[],
            parent: None,
        }
    }
}
impl<'a, It> Iterator for NodeIter<'a, It> {
    type Item = &'a It;

    fn next(&mut self) -> Option<Self::Item> {
        match self.child.first() {
            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(Node::Leaf(item)) => {
                self.child = &self.child[1..];
                Some(item)
            }
            Some(Node::Children(children)) => {
                self.child = &self.child[1..];

                *self = NodeIter {
                    child: children.as_slice(),
                    parent: Some(Box::new(mem::take(self))),
                };
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]

    fn test_tree_struct() {
        let tree = Node::Children(vec![
            Node::Leaf(1),
            Node::Leaf(2),
            Node::Children(vec![Node::Leaf(3), Node::Leaf(4), Node::Children(vec![])]),
        ]);

        let nums: Vec<i32> = tree.iter().copied().collect();
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }
}
