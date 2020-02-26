use std::fmt::Display;
#[allow(unused)]
use std::string::ToString;

const TRAVERSE_SYMBOL: &str = "│  ";
const TWO_WAY_SYMBOL: &str = "├──";
const ONE_WAY_SYMBOL: &str = "└──";

pub struct Tree<T> {
    root: Option<Node<T>>,
}

impl<T> Tree<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) -> &mut Self {
        match &mut self.root {
            Some(root) => root.insert(value),
            None => self.root = Some(Node::new(value)),
        };
        self
    }

    pub fn to_string(&self) -> String
    where
        T: Display,
    {
        let root = match &self.root {
            Some(node) => node,
            None => return String::from(""),
        };
        let mut sb = root.to_string();
        let left_pointer = match root.right.is_some() {
            true => TWO_WAY_SYMBOL,
            false => ONE_WAY_SYMBOL,
        };
        if let Some(left) = &root.left {
            left.show_into(&mut sb, "", left_pointer, root.right.is_some());
        }
        if let Some(right) = &root.right {
            right.show_into(&mut sb, "", ONE_WAY_SYMBOL, false);
        }
        sb
    }
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn show_into(&self, sb: &mut String, padding: &str, pointer: &str, has_right_leaf: bool)
    where
        T: Display,
    {
        sb.push_str(&format!("\n{}{}{}", padding, pointer, self));
        let mut padding_str = String::from(padding);

        match has_right_leaf {
            true => padding_str.push_str(TRAVERSE_SYMBOL),
            false => padding_str.push_str("   "),
        }
        let left_pointer = match self.right.is_some() {
            true => TWO_WAY_SYMBOL,
            false => ONE_WAY_SYMBOL,
        };
        if let Some(left) = &self.left {
            left.show_into(sb, &padding_str, left_pointer, self.right.is_some())
        }
        if let Some(right) = &self.right {
            right.show_into(sb, &padding_str, ONE_WAY_SYMBOL, false)
        }
    }

    pub fn insert(&mut self, value: T) {
        if value > self.value {
            self.insert_right(value);
        } else if value < self.value {
            self.insert_left(value);
        }
    }

    fn insert_left(&mut self, value: T) {
        match &mut self.left {
            Some(leaf) => leaf.insert(value),
            None => self.left = Some(Box::new(Node::new(value))),
        };
    }

    fn insert_right(&mut self, value: T) {
        match &mut self.right {
            Some(leaf) => leaf.insert(value),
            None => self.right = Some(Box::new(Node::new(value))),
        };
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.value)
    }
}
