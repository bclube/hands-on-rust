use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new(
            "Steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what's your name?");
        let name = read_line();

        if name.is_empty() {
            break;
        }

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        if let Some(visitor) = known_visitor {
            visitor.greet_visitor()
        } else {
            println!("{} is not on the visitor list.", name);
            visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
    }
    println!("The final list of visitors: {:#?}", visitor_list);
}

fn read_line() -> String {
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    input_string.trim().to_lowercase()
}
