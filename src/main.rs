use bst::Tree;

fn main() {
    println!("Tree with integers");
    let mut int_tree = Tree::new();
    int_tree
        .insert(5)
        .insert(6)
        .insert(4)
        .insert(3)
        .insert(10)
        .insert(11)
        .insert(7)
        .insert(8)
        .insert(2)
        .insert(1);
    println!("{}", int_tree.to_string());
    println!("\nTree with strings");
    let mut str_tree = Tree::new();
    str_tree
        .insert("node5")
        .insert("node7")
        .insert("node3")
        .insert("node4")
        .insert("node2")
        .insert("node1")
        .insert("node8")
        .insert("node9");
    println!("{}", str_tree.to_string());
}
