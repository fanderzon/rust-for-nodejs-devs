use store::Action::{ Visibility };
use rustc_serialize::json::{self, Json, ToJson};
use todo::{ Todo, TodoAction, todo_reducer };

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

#[derive(Clone, Debug)]
pub enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub enum VisibilityFilter {
    ShowActive,
    ShowAll,
    ShowCompleted,
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
