use std::fs;
use std::path::Path;
use tree_sitter::{Node, Tree};

/// A simple Parser tool which is use tree-sitter library
pub struct ParserUtility {}

impl ParserUtility {
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
    pub fn find_methods(
        root_node: Node,
        source_code: &str,
    ) -> Vec<(String, String, String, String)> {
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
                                                    .map(|n| {
                                                        source_code[n.byte_range()].to_string()
                                                    })
                                                    .unwrap_or("".to_string());

                                                let p_type = p
                                                    .child_by_field_name("type")
                                                    .map(|n| {
                                                        source_code[n.byte_range()].to_string()
                                                    })
                                                    .unwrap_or("object".to_string());

                                                parameters.push(format!("{} {}", p_type, &p_name));
                                            }
                                        }
                                    }
                                }

                                let param_str = parameters.join(", ");
                                methods.push((
                                    class_name.clone(),
                                    method_name,
                                    return_type,
                                    param_str,
                                ));
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
                                    let property_type =
                                        source_code[prop_type.byte_range()].to_string();
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

    /// Convert C# file into Abstract Syntax Tree
    pub fn parse_file(file_path: &str) -> Option<Tree> {
        if !file_path.ends_with(".cs") {
            eprintln!("Only works with C# files!");
            return None;
        }

        let source_code = fs::read_to_string(file_path).expect("File read error!");
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_c_sharp::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("Exception on loading C#");

        parser.parse(source_code, None)
    }

    /// Write Interface to a file from C# class
    pub fn generate_interface_from_file(file_path: &str) {
        if let Some(tree) = Self::parse_file(file_path) {
            let source_code = fs::read_to_string(file_path).expect("File read error!");
            let root_node = tree.root_node();
            let class_names = Self::find_classes(root_node, &source_code);
            let methods = Self::find_methods(root_node, &source_code);
            let properties = Self::find_properties(root_node, &source_code);

            if class_names.is_empty() {
                eprintln!("File not found!");
                return;
            }

            let interface_code = Self::generate_interface(&class_names[0], &methods, &properties);

            let interface_dir = Path::new("./interfaces");
            if !interface_dir.exists() {
                fs::create_dir(interface_dir).expect("Cannot create interfaces directory!");
            }

            let interface_filename = format!("./interfaces/I{}.cs", class_names[0]);
            fs::write(&interface_filename, interface_code).expect("File write error!");

            // dbg!("{} has been created.", interface_filename);
        }
    }
}
