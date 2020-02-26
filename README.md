# Simple generic BST implementation in Rust

Algorithm of visual representation taken from here: https://www.baeldung.com/java-print-binary-tree-diagram and adopted from Java to Rust with some improvements

## Usage Example
```rust
use bst::Tree;

fn main() {
   let tree = Tree::new();
   tree
      .insert(5, "A")
      .insert(2, "B");
   println!("{}", tree.to_string());
   match tree.search(2) {
      Some(node) => println!("{}", node.to_string()),
      None => println!("Key not found"),
   };
}
```

## Sample output from `main.rs`:
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
Search for key 'person7'
[person7(Azriel)]
```

## Roadmap
- [x] Insert
- [x] Search
- [x] Preorder Traversal
- [ ] Inorder Traversal
- [ ] Postorder Traversal
- [ ] Rebalance
