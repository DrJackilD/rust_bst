use std::fmt::Display;
#[allow(unused)]
use std::string::ToString;

const TRAVERSE_SYMBOL: &str = "║  ";
const EMPTY_TRAVERSE: &str = "   ";
const TWO_WAY_SYMBOL: &str = "╠══";
const ONE_WAY_SYMBOL: &str = "╚══";

pub struct Tree<T, V> {
    root: Option<Node<T, V>>,
}

impl<T, V> Tree<T, V>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: T, value: V) -> &mut Self {
        match &mut self.root {
            Some(root) => root.insert(key, value),
            None => self.root = Some(Node::new(key, value)),
        };
        self
    }

    pub fn search(&self, key: T) -> Option<&Node<T, V>> {
        match &self.root {
            Some(root) => root.search(key),
            None => None,
        }
    }

    pub fn to_string(&self) -> String
    where
        T: Display,
        V: Display,
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
pub struct Node<T, V> {
    key: T,
    value: V,
    left: Option<Box<Node<T, V>>>,
    right: Option<Box<Node<T, V>>>,
}

impl<T, V> Node<T, V>
where
    T: PartialOrd,
{
    pub fn new(key: T, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn show_into(&self, sb: &mut String, padding: &str, pointer: &str, has_right_leaf: bool)
    where
        T: Display,
        V: Display,
    {
        sb.push_str(&format!("\n{}{}{}", padding, pointer, self));
        let mut padding_str = String::from(padding);

        match has_right_leaf {
            true => padding_str.push_str(TRAVERSE_SYMBOL),
            false => padding_str.push_str(EMPTY_TRAVERSE),
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

    pub fn insert(&mut self, key: T, value: V) {
        if key > self.key {
            self.insert_right(key, value);
        } else if key < self.key {
            self.insert_left(key, value);
        }
    }

    fn insert_left(&mut self, key: T, value: V) {
        match &mut self.left {
            Some(leaf) => leaf.insert(key, value),
            None => self.left = Some(Box::new(Node::new(key, value))),
        };
    }

    fn insert_right(&mut self, key: T, value: V) {
        match &mut self.right {
            Some(leaf) => leaf.insert(key, value),
            None => self.right = Some(Box::new(Node::new(key, value))),
        };
    }

    pub fn search(&self, key: T) -> Option<&Self> {
        if key > self.key {
            match &self.right {
                Some(node) => return node.search(key),
                None => None,
            }
        } else if key == self.key {
            return Some(self);
        } else {
            match &self.left {
                Some(node) => node.search(key),
                None => None,
            }
        }
    }
}

impl<T, V> Display for Node<T, V>
where
    T: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}({})]", self.key, self.value)
    }
}
