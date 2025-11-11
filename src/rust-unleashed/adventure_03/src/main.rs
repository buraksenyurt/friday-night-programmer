#![allow(dead_code)]

fn main() {
    // scenario_1();
    scenario_2();
    println!("{}", "*".repeat(20));
    scenario_3();

    // println!("End of the programme");
}

fn scenario_1() {
    let dotenv = Process {
        id: ProcessId(1903),
        is_active: false,
        name: "Dot Net Environment".to_string(),
    };
    print_process(&dotenv);
}

fn print_process(process: &Process) {
    println!("{:?}", process);
}

fn scenario_2() {
    let Process { name, .. } = Process {
        id: ProcessId(1905),
        is_active: false,
        name: "Docker Compose".to_string(),
    };
    println!("{:?}", name);
}

fn scenario_3() {
    let Process { ref name, .. } = Process {
        id: ProcessId(1905),
        is_active: false,
        name: "Docker Compose".to_string(),
    };
    println!("{:?}", name);
}

#[derive(Debug)]
struct ProcessId(u16);

impl Drop for ProcessId {
    fn drop(&mut self) {
        println!("Droping process id {}", self.0);
    }
}

#[derive(Debug)]
struct Process {
    id: ProcessId,
    is_active: bool,
    name: String,
}

// impl Drop for Process {
//     fn drop(&mut self) {
//         println!("Droping process {}", self.name);
//     }
// }
