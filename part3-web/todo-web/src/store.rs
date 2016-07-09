use store::TodoAction::{ Add, Remove, Toggle };
use store::Action::{ Todos, Visibility };
use store::VisibilityFilter:: { ShowActive, ShowAll, ShowCompleted };

use rustc_serialize::json::{self, Json, ToJson};

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct State {
    pub todos: Vec<Todo>,
    pub visibility_filter: VisibilityFilter
}
impl State {
    // This gives us a quick way to initialize a default state with State::default()
    pub fn default() -> State {
        State {
            todos: Vec::new(),
            visibility_filter: VisibilityFilter::ShowAll,
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        Json::from_str( &json::encode(&self).unwrap() ).unwrap()
    }
}

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

#[derive(Clone, Debug)]
pub enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

// mark_done from the previous example becomes Toggle to align with the Redux example
// otherwise functionality is the same
#[derive(Clone, Debug)]
pub enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub enum VisibilityFilter {
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
pub fn reducer(state: &State, action: Action) -> State {
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
pub struct Store {
    state: State,
    listeners: Vec<fn(&State)>,
    reducer: fn(&State, Action) -> State,
}

impl Store {
    // Takes a reducer function, we skip the initial_state and optional arguments
    // TO keep it simple, State::default() from earlier is our initial_state implementation
    pub fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            listeners: Vec::new(),
            reducer: reducer,
        }
    }

    // Pushes a listener that will be called for any state change
    #[allow(dead_code)]
    pub fn subscribe(&mut self, listener: fn(&State)) {
        self.listeners.push(listener);
    }

    // Simply returns the state
    #[allow(dead_code)]
    pub fn get_state(&self) -> &State {
        &self.state
    }

    // Called for every new action, calls the reducer to update the state
    // and then calls every listener
    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}
