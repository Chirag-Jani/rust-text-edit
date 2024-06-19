use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RopeNode {
    Leaf {
        value: String,
        weight: usize,
    },
    Internal {
        left: Box<RopeNode>,
        right: Box<RopeNode>,
        weight: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rope {
    root: RopeNode,
}

impl Rope {
    pub fn new(val: &str) -> Self {
        Self {
            root: RopeNode::Leaf {
                value: val.to_string(),
                weight: val.len(),
            },
        }
    }

    pub fn concat(left: Rope, right: Rope) -> Result<Self, Error> {
        let left_weight = left.weight_recursive(&left.root);

        let new_root = Self {
            root: RopeNode::Internal {
                left: Box::new(left.root),
                right: Box::new(right.root),
                weight: left_weight,
            },
        };

        Ok(new_root)
    }

    pub fn weight_recursive(&self, node: &RopeNode) -> usize {
        match node {
            RopeNode::Internal { left, .. } => {
                //
                let wt = self.weight_recursive(left.as_ref());
                wt
            }
            RopeNode::Leaf { value, .. } => value.len(),
        }
    }

    pub fn char_at_index(&self, index: usize) -> Result<Option<char>, Error> {
        let answer = self.find_at_index(&self.root, index).unwrap();
        Ok(answer)
    }

    pub fn find_at_index(&self, rope: &RopeNode, index: usize) -> Result<Option<char>, Error> {
        match rope {
            RopeNode::Leaf { value, .. } => {
                let ans = value.chars().nth(index).unwrap();
                return Ok(Some(ans));
            }
            RopeNode::Internal {
                left,
                right,
                weight,
            } => {
                if index < *weight {
                    self.find_at_index(&left, index)
                } else {
                    self.find_at_index(&right, index - *weight)
                }
            }
        }
    }

    pub fn split_at_index(&self, index: usize) -> (Option<RopeNode>, Option<RopeNode>) {
        self.split_recursive(&self.root, index)
    }

    fn split_recursive(
        &self,
        node: &RopeNode,
        index: usize,
    ) -> (Option<RopeNode>, Option<RopeNode>) {
        match node {
            RopeNode::Leaf { value, .. } => {
                //
                let left = RopeNode::Leaf {
                    value: value[..index].to_string(),
                    weight: value[..index].len(),
                };
                let right = RopeNode::Leaf {
                    value: value[index..].to_string(),
                    weight: value[index..].len(),
                };
                (Some(left), Some(right))
            }
            RopeNode::Internal {
                left,
                right,
                weight,
            } => {
                //
                if index < *weight {
                    let (l, r) = self.split_recursive(&left, index);
                    return (l, r);
                } else {
                    let (l, r) = self.split_recursive(&right, index - *weight);
                    return (l, r);
                }
            }
        }
    }

    pub fn insert_at_index(&self, index: usize, val: &str) -> Result<Rope, Error> {
        let (l1, r1) = self.split_at_index(index);
        let middle = Rope {
            root: RopeNode::Leaf {
                value: val.to_string(),
                weight: val.len(),
            },
        };

        let c1 = Rope::concat(Rope { root: l1.unwrap() }, middle).unwrap();
        let c2 = Rope::concat(c1, Rope { root: r1.unwrap() }).unwrap();
        Ok(c2)
    }

    pub fn delete_between_index(&self, start: usize, end: usize) -> Result<Rope, Error> {
        let (n1, _n2) = self.split_at_index(start);
        let (_n3, n4) = self.split_at_index(end);

        let left = Rope { root: n1.unwrap() };
        let right = Rope { root: n4.unwrap() };

        let answer = Rope::concat(left, right).unwrap();

        Ok(answer)
    }
}
