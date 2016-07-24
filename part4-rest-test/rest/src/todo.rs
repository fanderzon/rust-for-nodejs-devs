use store::{ Action };
use store::Action::{ Todos };
use todo::TodoAction::{ Add, Toggle, Remove };
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Todo {
    pub id: i16,
    pub title: String,
    pub completed: bool,
    pub deleted: bool,
}
impl Todo {
    pub fn new(id: i16, title: String) -> Todo {
        Todo {
            id: id,
            title: title,
            completed: false,
            deleted: false,
        }
    }
}

// mark_done from the previous example becomes Toggle to align with the Redux example
// otherwise functionality is the same
#[derive(Clone, Debug)]
pub enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

// Helper function for getting a mutable todo from a vector by todo_id
pub fn get_mut_todo(todos: &mut Vec<Todo>, todo_id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo|todo.id == todo_id)
}

// Our todo reducer, takes in state (todo list) and returns a new/cloned version
// after applying the action (is applicable)
pub fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
    let mut new_state: Vec<Todo> = state.clone();

    // First we make sure it's a `Todos` action, otherwise return clone of incoming state
    match *action {
        Todos(ref todo_action) => match *todo_action {
            // Pretty simple from here on, check the type of Todos enum type
            // If Add push a new item, and if `Toggle` or `Remove` use our get_mut_todo
            // helper function and then change a property on the todo
            Add(ref title) => {
                let new_id = new_state.len() as i16 + 1;
                new_state.push(Todo::new(new_id, title.to_string()))
            },
            Toggle(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    if todo.completed { todo.completed = false; } else { todo.completed = true; }
                }
            },
            Remove(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    todo.deleted = true;
                }
            },
        },
        // If it's not a Todos action change nothing
        _ => (),
    }
    return new_state;
}
