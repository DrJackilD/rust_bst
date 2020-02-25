const TRAVERSE_SYMBOL: &str = "│  ";
const TWO_WAY_SYMBOL: &str = "├──";
const ONE_WAY_SYMBOL: &str = "└──";

pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T>
where
    T: std::fmt::Display,
{
    pub fn new(root: Node<T>) -> Tree<T> {
        Tree { root }
    }

    pub fn show(&self) -> String {
        let mut sb = String::new();

        sb.push_str(&format!("{}", self.root));
        let left_pointer = if self.root.right.is_some() {
            TWO_WAY_SYMBOL
        } else {
            ONE_WAY_SYMBOL
        };
        match &self.root.left {
            Some(left) => left.show(&mut sb, "", left_pointer, self.root.right.is_some()),
            None => (),
        }
        match &self.root.right {
            Some(right) => right.show(&mut sb, "", ONE_WAY_SYMBOL, false),
            None => (),
        }
        sb
    }
}

#[derive(Debug, Default)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: std::fmt::Display,
{
    fn show(&self, sb: &mut String, padding: &str, pointer: &str, has_right_leaf: bool) {
        sb.push_str(&format!("\n{}{}{}", padding, pointer, self));
        let mut padding_str = String::from(padding);
        match has_right_leaf {
            true => padding_str.push_str(TRAVERSE_SYMBOL),
            false => padding_str.push_str("   "),
        }
        let left_pointer = if self.right.is_some() {
            TWO_WAY_SYMBOL
        } else {
            ONE_WAY_SYMBOL
        };
        match &self.left {
            Some(left) => left.show(sb, &padding_str, left_pointer, self.right.is_some()),
            None => (),
        }
        match &self.right {
            Some(right) => right.show(sb, &padding_str, ONE_WAY_SYMBOL, false),
            None => (),
        }
    }
}

impl<T> Node<T>
where
    T: std::default::Default,
{
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            ..Default::default()
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

impl<T> std::fmt::Display for Node<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.value)
    }
}
