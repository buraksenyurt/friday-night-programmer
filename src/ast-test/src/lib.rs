use tree_sitter::{Parser, Tree};

fn parse(source_code: &str) -> Tree {
    let mut parser = Parser::new();
    let language = tree_sitter_c_sharp::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Exception on loading C#");
    parser.parse(source_code, None).expect("Parsing error")
}

#[test]
fn test_find_classes() {
    let code = r#"
        public class GameEngine {
            public void Build() {
                Console.WriteLine("Initiating...");
            }
        }

        public class Customer {
            public int CustomerId {get; set;}
        }

        public interface IEntity {
        
        }
    "#;

    let tree = parse(code);
    let root_node = tree.root_node();
    let mut found_class = false;

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            found_class = true;
            assert!(node.child_by_field_name("name").is_some());
        }
    }

    assert!(found_class, "No class found in AST!");
}
