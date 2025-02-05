fn main() {
    let fpath = file!();
    println!("filepath: {0}", fpath);

    let content = "[1, {\"key1\": \"val1\", \"key2\": \"val2\"}]";

    let mut parser = tree_sitter::Parser::new();

    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("error handling rust grammar");
    let tree = parser.parse(content, None).unwrap();
    let root_node = tree.root_node();
    println!("kind: {}", root_node.kind());
    let array_node = root_node.named_child(0).unwrap();
    println!("kind: {}", array_node.kind());
    let number_node = array_node.named_child(0).unwrap();
    println!("kind: {}", number_node.kind());
    let object_node = array_node.named_child(1).unwrap();
    println!("kind: {}", object_node.kind());
    assert_eq!(root_node.child_count(), 1);
    assert_eq!(array_node.child_count(), 5);
    assert_eq!(array_node.named_child_count(), 2);
    assert_eq!(number_node.child_count(), 0);
    assert_eq!(object_node.child_count(), 5);
    println!("syntax tree: {0}", root_node.to_sexp());
    let mut cursor = root_node.walk();
    for nc in root_node.named_children(&mut cursor) {
        println!("kind: {0}", nc.kind());
    }

    let pk_node = object_node.named_child(0).unwrap();
    let mut pk_cursor = pk_node.walk();
    for k_node in pk_node.children_by_field_name("key", &mut pk_cursor) {
        println!("k_node: {}", k_node);
    }

    if let Some(d) = root_node.child_with_descendant(root_node) {
        println!("descendant: {}, has_error: {}", d, d.has_error());
        assert_eq!(d, array_node);
    } else {
        println!("cannot find descendant");
    }
}
