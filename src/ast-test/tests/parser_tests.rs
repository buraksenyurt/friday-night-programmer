use ast_test::parser_utility::*;

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

    let tree = ParserUtility::parse(code).expect("Can't parse abstract syntax tree");
    let root_node = tree.root_node();
    let class_names = ParserUtility::find_classes(root_node, code);

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

    let tree = ParserUtility::parse(code).expect("Can't parse abstract syntax tree");
    let root_node = tree.root_node();
    let methods = ParserUtility::find_methods(root_node, code);

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

    let tree = ParserUtility::parse(code).expect("Can't parse abstract syntax tree");
    let root_node = tree.root_node();
    let class_names = ParserUtility::find_classes(root_node, code);
    let methods = ParserUtility::find_methods(root_node, code);
    let properties = ParserUtility::find_properties(root_node, code);

    let interface_code = ParserUtility::generate_interface(&class_names[0], &methods, &properties);

    let expected_interface = r#"
public interface ICustomerBusiness {
    bool OnTestMode { get; set; }
    bool AddBonus(int customerId, double amount);
    Customer GetCustomer(int customerId);
}
"#;

    assert_eq!(interface_code.trim(), expected_interface.trim());
}
