use bst::Tree;

fn main() {
    println!("Tree with integers");
    let mut int_tree = Tree::new();
    int_tree
        .insert(5, "Hello world")
        .insert(6, "Traits")
        .insert(4, "Boundary")
        .insert(3, "Variables")
        .insert(10, "Closures")
        .insert(11, "Lifetimes")
        .insert(7, "Borrowing")
        .insert(8, "Functiona programming")
        .insert(2, "Cargo")
        .insert(1, "Crates");
    println!("{}", int_tree.to_string());
    println!("\nTree with strings");
    let mut str_tree = Tree::new();
    str_tree
        .insert("person5", "Sam")
        .insert("person2", "Frodo")
        .insert("person8", "Gandalf")
        .insert("person6", "Boromyr")
        .insert("person10", "Aragorn")
        .insert("person11", "Elf")
        .insert("person9", "Bilbo")
        .insert("person3", "Aramyr")
        .insert("person4", "Borogor")
        .insert("person7", "Azriel");
    println!("{}", str_tree.to_string());

    println!("Search for key 'person7'");
    let result = match str_tree.search("person7") {
        Some(node) => node.to_string(),
        None => String::from("Key not found"),
    };
    println!("{}", result);
}
