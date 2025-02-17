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
    let mut class_names = Vec::new();

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            if let Some(name) = node.child_by_field_name("name") {
                class_names.push(code[name.byte_range()].to_string());
            }
        }
    }

    assert_eq!(
        class_names,
        vec!["GameEngine".to_string(), "Customer".to_string()]
    );
}

//TODO@buraksenyurt Extend with return type and method parameters
#[test]
fn test_find_methods() {
    let code = r#"
        public class Einstein {
            public double Add(double x, double y) {
                return x + y;
            }

            public double Mul(double x, double y) {
                return x * y;
            }

            public void ClearTemp() {
                Console.WriteLine("Clearing temp folder...");
            }

            public int[] GetRandomNumbers(int maxCount) {
                return new int[] {1,2,3,4};
            }
        }
    "#;

    let tree = parse(code);
    let root_node = tree.root_node();
    let mut methods = Vec::new();

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            if let Some(class_name_node) = node.child_by_field_name("name") {
                let class_name = code[class_name_node.byte_range()].to_string();

                if let Some(body) = node.child_by_field_name("body") {
                    for member in body.children(&mut body.walk()) {
                        if member.kind() == "method_declaration" {
                            let method_name = member
                                .child_by_field_name("name")
                                .map(|n| code[n.byte_range()].to_string())
                                .unwrap_or("Unknown".to_string());

                            methods.push((class_name.clone(), method_name));
                        }
                    }
                }
            }
        }
    }

    assert_eq!(
        methods,
        vec![
            ("Einstein".to_string(), "Add".to_string(),),
            ("Einstein".to_string(), "Mul".to_string(),),
            ("Einstein".to_string(), "ClearTemp".to_string(),),
            ("Einstein".to_string(), "GetRandomNumbers".to_string(),)
        ]
    );
}
