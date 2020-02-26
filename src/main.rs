use bst::{Node, Tree};

fn main() {
    let root = Node::new("root")
        .left(
            Node::new("node1")
                .left(Node::new("node3"))
                .right(Node::new("node4")),
        )
        .right(
            Node::new("node2")
                .left(
                    Node::new("node5").left(
                        Node::new("node7")
                            .left(Node::new("node8"))
                            .right(Node::new("node9")),
                    ),
                )
                .right(Node::new("node6")),
        );
    let tree = Tree::new(root);
    println!("{}", tree.to_string());
}
