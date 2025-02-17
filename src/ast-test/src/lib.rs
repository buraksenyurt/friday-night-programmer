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

    let mut class_name = String::new();
    let mut methods = Vec::new();
    let mut properties = Vec::new();

    for node in root_node.children(&mut root_node.walk()) {
        if node.kind() == "class_declaration" {
            if let Some(name) = node.child_by_field_name("name") {
                class_name = format!("I{}", code[name.byte_range()].to_string());
            }

            if let Some(body) = node.child_by_field_name("body") {
                for member in body.children(&mut body.walk()) {
                    if member.kind() == "method_declaration" {
                        let return_type = member
                            .child_by_field_name("returns")
                            .map(|n| code[n.byte_range()].to_string())
                            .unwrap_or("".to_string());
                        // dbg!(return_type);

                        let method_name = member
                            .child_by_field_name("name")
                            .map(|n| code[n.byte_range()].to_string())
                            .unwrap_or("".to_string());
                        // dbg!(method_name);

                        let mut parameters = Vec::new();

                        for cm in member.children(&mut member.walk()) {
                            if cm.kind() == "parameter_list" {
                                for p in cm.children(&mut cm.walk()) {
                                    if p.kind() == "parameter" {
                                        let p_name = p
                                            .child_by_field_name("name")
                                            .map(|n| code[n.byte_range()].to_string())
                                            .unwrap_or("".to_string());

                                        let p_type = p
                                            .child_by_field_name("type")
                                            .map(|n| code[n.byte_range()].to_string())
                                            .unwrap_or("object".to_string());

                                        // dbg!(format!("{} {}", p_type, &p_name));

                                        parameters.push(format!("{} {}", p_type, &p_name));
                                    }
                                }
                            }
                        }
                        let param_str = parameters.join(", ");
                        methods.push(format!(
                            "    {} {}({});",
                            return_type, method_name, param_str
                        ));
                    } else if member.kind() == "property_declaration" {
                        if let Some(name) = member.child_by_field_name("name") {
                            if let Some(prop_type) = member.child_by_field_name("type") {
                                let property_name = code[name.byte_range()].to_string();
                                let property_type = code[prop_type.byte_range()].to_string();
                                properties.push(format!(
                                    "    {} {} {{ get; set; }}",
                                    property_type, property_name
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut interface_code = format!("public interface {} {{\n", class_name);
    for prop in properties {
        interface_code.push_str(&format!("{}\n", prop));
    }
    for method in methods {
        interface_code.push_str(&format!("{}\n", method));
    }
    interface_code.push_str("}\n");

    let expected_interface = r#"
public interface ICustomerBusiness {
    bool OnTestMode { get; set; }
    bool AddBonus(int customerId, double amount);
    Customer GetCustomer(int customerId);
}
"#;

    assert_eq!(interface_code.trim(), expected_interface.trim());
}
