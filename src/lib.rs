use std::fmt::Display;
use std::string::ToString;

const TRAVERSE_SYMBOL: &str = "│  ";
const TWO_WAY_SYMBOL: &str = "├──";
const ONE_WAY_SYMBOL: &str = "└──";

pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    pub fn new(root: Node<T>) -> Self {
        Self { root }
    }

    pub fn to_string(&self) -> String
    where
        T: Display,
    {
        let mut sb = self.root.to_string();
        let left_pointer = match self.root.right.is_some() {
            true => TWO_WAY_SYMBOL,
            false => ONE_WAY_SYMBOL,
        };
        if let Some(left) = &self.root.left {
            left.show_into(&mut sb, "", left_pointer, self.root.right.is_some());
        }
        if let Some(right) = &self.root.right {
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

impl<T> Node<T> {
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
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, leaf: Node<T>) -> Self {
        self.left = Some(Box::new(leaf));
        self
    }

    pub fn right(mut self, leaf: Node<T>) -> Self {
        self.right = Some(Box::new(leaf));
        self
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
