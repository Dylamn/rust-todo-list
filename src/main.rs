// 1. Add a task:
//    Allow the user to add a task which has :
//     - title
//     - description
// 2. Display the tasks list:
//    Show if they're done or not.
// 3. Mark a task as completed:
//    Allow the user to mark a task as completed
// 4. Delete a task:
//    Allow the user to delete a specific task from the list
// 5. Leave the application:
//    Allow the user to leave the application
fn main() {
    println!("Hello and welcome to To-do List!");

    let mut tasks: Vec<[String; 3]> = vec![];

    loop {
        match create_todo() {
            Some(val) => {
                tasks.push(val);
            }
            None => {
                println!("Goodbye!");
                // The user has written "exit".
                break;
            }
        }

        println!("Here's your to-do list:");
        for (index, todo) in tasks.iter().enumerate() {
            // The '2' comes from the ": " printed after the index.
            let offset = String::from(" ").repeat(index.to_string().len() + 2);
            let task_num = index + 1;
            let [title, desc, status] = todo;

            println!("{}: {} ({})\n{offset}{}", task_num, title, status, desc);
        }

        let todo_complete_index = 0;
    }
}

fn mark_todo_as_completed() {

}
fn create_todo() -> Option<[String; 3]> {
    let todo_title = match get_user_input("Please enter a name for your task ('quit' to exit):") {
        Some(input) => input,
        None => {
            return None;
        }
    };

    let todo_description =
        match get_user_input("Please enter a description for your task ('quit' to exit):") {
            Some(input) => input,
            None => {
                return None;
            }
        };

    Some([todo_title, todo_description, String::from("Not done")])
}

fn get_user_input(display_text: &str) -> Option<String> {
    let mut user_input = String::new();

    loop {
        println!("{}", display_text);

        match std::io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input = user_input.trim().to_string();
                break;
            }
            Err(_) => {
                println!("Invalid input. Please try again.");
            }
        }
    }
    if user_input == "quit" || user_input == "exit" {
        return None;
    }

    Some(user_input.trim().to_string())
}
