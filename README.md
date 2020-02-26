# Simple generic BST implementation in Rust

Algorithm of visual representation taken from here: https://www.baeldung.com/java-print-binary-tree-diagram and adopted from Java to Rust with some improvements

## Usage Example
```rust
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
```

### Sample output:
```
Tree with integers
[5]
├──[4]
│  └──[3]
│     └──[2]
│        └──[1]
└──[6]
   └──[10]
      ├──[7]
      │  └──[8]
      └──[11]

Tree with strings
[node5]
├──[node3]
│  ├──[node2]
│  │  └──[node1]
│  └──[node4]
└──[node7]
   └──[node8]
      └──[node9]
```

## Roadmap
- [x] Insert
- [ ] Search
- [x] Preorder Traversal
- [ ] Inorder Traversal
- [ ] Postorder Traversal
- [ ] Rebalance
