# Simple generic BST implementation in Rust

Algorithm of visual representation taken from here: https://www.baeldung.com/java-print-binary-tree-diagram and adopted from Java to Rust with some improvements

## Usage Example
```rust
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
}
```

### Sample output:
```
Tree with integers
[5(Hello world)]
╠══[4(Boundary)]
║  ╚══[3(Variables)]
║     ╚══[2(Cargo)]
║        ╚══[1(Crates)]
╚══[6(Traits)]
   ╚══[10(Closures)]
      ╠══[7(Borrowing)]
      ║  ╚══[8(Functiona programming)]
      ╚══[11(Lifetimes)]

Tree with strings
[person5(Sam)]
╠══[person2(Frodo)]
║  ╠══[person10(Aragorn)]
║  ║  ╚══[person11(Elf)]
║  ╚══[person3(Aramyr)]
║     ╚══[person4(Borogor)]
╚══[person8(Gandalf)]
   ╠══[person6(Boromyr)]
   ║  ╚══[person7(Azriel)]
   ╚══[person9(Bilbo)]
```

## Roadmap
- [x] Insert
- [ ] Search
- [x] Preorder Traversal
- [ ] Inorder Traversal
- [ ] Postorder Traversal
- [ ] Rebalance
