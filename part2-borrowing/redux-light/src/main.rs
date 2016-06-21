use std::io;

// Lets us type `Add("Todo item".to_string())` instead of `TodoAction::Add("Todo item".to_string())`
use TodoAction::{ Add, Remove, Toggle };
// Same with the Action enum and VisibilityFilter, Action::*; would work too, but this way we list what we use
use Action::{ Todos, Visibility };
use VisibilityFilter:: { ShowActive, ShowAll, ShowCompleted };

// Ripping off the canonical Redux todo example we'll add a
// visibility filter to our state except for the todos we already had
#[derive(Clone, Debug)]
struct State {
    todos: Vec<Todo>,
    visibility_filter: VisibilityFilter
}

// By implementing a struct we are creating something very much like
// a class, we can attach methods to it refering to &self` or `&mut self`
impl State {
    // This gives us a quick way to initialize a default state with State::default()
    pub fn default() -> State {
        State {
            todos: Vec::new(),
            visibility_filter: VisibilityFilter::ShowAll,
        }
    }
}

// Same Todo as last time..
#[derive(Clone, Debug)]
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

// Create a convenient Todo::new(id, title) method
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


// Rust has enums, so the enum type can replace the "type" property of Redux objects
// The enums will replace `action creators` too since Todos(Add("Todo item".to_string()))
// is pretty clear
#[derive(Clone, Debug)]
enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

// mark_done from the previous example becomes Toggle to align with the Redux example
// otherwise functionality is the same
#[derive(Clone, Debug)]
enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

// Our 3 visibility states
#[derive(Clone, Debug)]
enum VisibilityFilter {
    ShowActive,
    ShowAll,
    ShowCompleted,
}

// Helper function for getting a mutable todo from a vector by todo_id
fn get_mut_todo(todos: &mut Vec<Todo>, todo_id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo|todo.id == todo_id)
}

// Our main reducer, returns a new State with the results of the child-reducers
// No combineReducers is implemented here, so it calls the child reducers
// by function name
fn reducer(state: &State, action: Action) -> State {
    // Always return a new state
    State {
        todos: todo_reducer(&state.todos, &action),
        visibility_filter: visibility_reducer(&state.visibility_filter, &action),
    }
}

// Our todo reducer, takes in state (todo list) and returns a new/cloned version
// after applying the action (is applicable)
fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
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

// Very simple reducer since the action will either be a VisibilityFilter, in which
// case we will return that, otherwise just return the incoming state
fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match *action {
        Visibility(ref vis_action) => vis_action.clone(),
        _ => state.clone(),
    }
}

// Redux store implementation
struct Store {
    state: State,
    listeners: Vec<fn(&State)>,
    reducer: fn(&State, Action) -> State,
}

impl Store {
    // Takes a reducer function, we skip the initial_state and optional arguments
    // TO keep it simple, State::default() from earlier is our initial_state implementation
    fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            listeners: Vec::new(),
            reducer: reducer,
        }
    }

    // Pushes a listener that will be called for any state change
    fn subscribe(&mut self, listener: fn(&State)) {
        self.listeners.push(listener);
    }

    // Simply returns the state
    #[allow(dead_code)]
    fn get_state(&self) -> &State {
        &self.state
    }

    // Called for every new action, calls the reducer to update the state
    // and then calls every listener
    fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}

// Very simple function to print a todo
fn print_todo(todo: &Todo) {
    let done = if todo.completed { "✔" } else { " " };
    println!("[{}] {} {}", done, todo.id, todo.title);
}

// Our print_todos function from last time, a bit altered to take State
// as input instead of a todos list directly
fn print_todos(state: &State) {
    let visibility = &state.visibility_filter;
    println!("\n\nTodo List:\n-------------------");
    for todo in &state.todos {
        if !todo.deleted {
            match *visibility {
                ShowAll => print_todo(&todo),
                ShowCompleted => if todo.completed { print_todo(&todo) },
                ShowActive => if !todo.completed { print_todo(&todo) },
            }
        }
    }
    println!("-------------------\nVisibility filter:  {:?}", visibility);
    print_instructions();
}


fn print_instructions() {
    println!("\nAvailable commands: \nadd [text] - toggle [id] - remove [id]\nshow [all|active|completed]");
}

fn invalid_command(command: &str) {
    println!("Invalid command: {}", command);
}

fn main() {
    // Let's create our store and subscribe with print_todos so every update is printed
    let mut store = Store::create_store(reducer);
    store.subscribe(print_todos);

    print_instructions();

    // Same input handling as last time, the interesting parts will be in our match statement
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line");
        let command_parts: Vec<&str> = command.split_whitespace().collect();

        match command_parts.len() {
            0 => invalid_command(&command),
            _ => {
                match command_parts[0] {
                    // Since we prepared so well we just need to call dispatch on our store
                    // With the right action
                    "add" => store.dispatch( Todos(Add( command_parts[1..].join(" ").to_string() ))),
                    "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        store.dispatch( Todos(Remove(num)));
                    },
                    "toggle" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        store.dispatch( Todos(Toggle(num)));
                    },
                    "show" => match command_parts[1] {
                        "all" => store.dispatch( Visibility(ShowAll) ),
                        "active" => store.dispatch( Visibility(ShowActive) ),
                        "completed" => store.dispatch( Visibility(ShowCompleted) ),
                        _ => invalid_command(&command)
                    },
                    _ => invalid_command(&command),
                }
            },
        }
    }
}
