use std::fs;
use tree_sitter::{Node, Tree};

/// Convert C# source code into Abstract Syntax Tree-AST
pub fn parse(source_code: &str) -> Tree {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_c_sharp::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Exception on loading C#");
    parser.parse(source_code, None).expect("Parsing error")
}

/// Find class names in code
pub fn find_classes(root_node: Node, source_code: &str) -> Vec<String> {
    let mut class_names = Vec::new();

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            if let Some(name) = node.child_by_field_name("name") {
                class_names.push(source_code[name.byte_range()].to_string());
            }
        }
    }

    class_names
}

/// Find methods with name, parameters and return type
pub fn find_methods(root_node: Node, source_code: &str) -> Vec<(String, String, String, String)> {
    let mut methods = Vec::new();

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            if let Some(class_name_node) = node.child_by_field_name("name") {
                let class_name = source_code[class_name_node.byte_range()].to_string();

                if let Some(body) = node.child_by_field_name("body") {
                    for member in body.children(&mut body.walk()) {
                        if member.kind() == "method_declaration" {
                            let method_name = member
                                .child_by_field_name("name")
                                .map(|n| source_code[n.byte_range()].to_string())
                                .unwrap_or("Unknown".to_string());

                            let return_type = member
                                .child_by_field_name("returns")
                                .map(|n| source_code[n.byte_range()].to_string())
                                .unwrap_or("void".to_string());

                            let mut parameters = Vec::new();

                            for cm in member.children(&mut member.walk()) {
                                if cm.kind() == "parameter_list" {
                                    for p in cm.children(&mut cm.walk()) {
                                        if p.kind() == "parameter" {
                                            let p_name = p
                                                .child_by_field_name("name")
                                                .map(|n| source_code[n.byte_range()].to_string())
                                                .unwrap_or("".to_string());

                                            let p_type = p
                                                .child_by_field_name("type")
                                                .map(|n| source_code[n.byte_range()].to_string())
                                                .unwrap_or("object".to_string());

                                            parameters.push(format!("{} {}", p_type, &p_name));
                                        }
                                    }
                                }
                            }

                            let param_str = parameters.join(", ");
                            methods.push((class_name.clone(), method_name, return_type, param_str));
                        }
                    }
                }
            }
        }
    }

    methods
}

/// Find properties of C# class
pub fn find_properties(root_node: Node, source_code: &str) -> Vec<(String, String)> {
    let mut properties = Vec::new();

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            if let Some(body) = node.child_by_field_name("body") {
                for member in body.children(&mut body.walk()) {
                    if member.kind() == "property_declaration" {
                        if let Some(name) = member.child_by_field_name("name") {
                            if let Some(prop_type) = member.child_by_field_name("type") {
                                let property_name = source_code[name.byte_range()].to_string();
                                let property_type = source_code[prop_type.byte_range()].to_string();
                                properties.push((property_type, property_name));
                            }
                        }
                    }
                }
            }
        }
    }

    properties
}

/// Generate interface types from classes
pub fn generate_interface(
    class_name: &str,
    methods: &[(String, String, String, String)],
    properties: &[(String, String)],
) -> String {
    let mut interface_code = format!("public interface I{} {{\n", class_name);

    for (prop_type, prop_name) in properties {
        interface_code.push_str(&format!(
            "    {} {} {{ get; set; }}\n",
            prop_type, prop_name
        ));
    }

    for (_, method_name, return_type, params) in methods {
        interface_code.push_str(&format!(
            "    {} {}({});\n",
            return_type, method_name, params
        ));
    }

    interface_code.push_str("}\n");

    interface_code
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
    let class_names = find_classes(root_node, code);

    assert_eq!(
        class_names,
        vec!["GameEngine".to_string(), "Customer".to_string()]
    );
}

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
    let methods = find_methods(root_node, code);

    assert_eq!(
        methods,
        vec![
            (
                "Einstein".to_string(),
                "Add".to_string(),
                "double".to_string(),
                "double x, double y".to_string()
            ),
            (
                "Einstein".to_string(),
                "Mul".to_string(),
                "double".to_string(),
                "double x, double y".to_string()
            ),
            (
                "Einstein".to_string(),
                "ClearTemp".to_string(),
                "void".to_string(),
                "".to_string()
            ),
            (
                "Einstein".to_string(),
                "GetRandomNumbers".to_string(),
                "int[]".to_string(),
                "int maxCount".to_string()
            )
        ]
    );
}

#[test]
fn test_generate_interface() {
    let code = r#"
            public class CustomerBusiness : BusinessBase
            {
                public bool OnTestMode {get; set;}
                public bool AddBonus(int customerId, double amount)
                {
                    return true;
                }
                public Customer GetCustomer(int customerId){
                    return null;
                }
            }
    "#;

    let tree = parse(code);
    let root_node = tree.root_node();
    let class_names = find_classes(root_node, code);
    let methods = find_methods(root_node, code);
    let properties = find_properties(root_node, code);

    let interface_code = generate_interface(&class_names[0], &methods, &properties);

    let expected_interface = r#"
public interface ICustomerBusiness {
    bool OnTestMode { get; set; }
    bool AddBonus(int customerId, double amount);
    Customer GetCustomer(int customerId);
}
"#;

    assert_eq!(interface_code.trim(), expected_interface.trim());
}
