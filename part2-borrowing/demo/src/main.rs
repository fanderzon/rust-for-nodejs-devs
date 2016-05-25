// Let's reuse our Todo struct from last time
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

// Now the function takes the borrowed version `&Todo`
fn print_todo(todo: &Todo) {
    println!("Todo item {}: {}", todo.id, todo.title);
}

fn main() {
    // Create a todo
    let item0 = Todo {
    	id: 0,
        title: "Borrow something".to_string(),
        completed: false,
        deleted: false
    };
    // Use our little function to print a borrowed value
    print_todo(&item0);
    // This still moves the value to the borrow_something variable
    let borrow_something = item0;

    // All numeric primitives, signed and unsigned are copyable
    let a: i64 = 10;
    let b = a;
    let c: f32 = 0.1;
    let d = c;
    println!("a and c are not moved: {} {}", a, c);

    // Bools too
    let e = true;
    let f = e;
    println!("Bools are copyable too: {}", e);

    // Char's and string slices are copyable too, BUT NOT Strings
    let g: char = 'g';
    let h = g;
    let i = "string slices are copyable";
    let j = i;
    println!("chars are copyable: {} aaaand {}", g, i);

    // Arrays and array slices too
    let k = [1, 2, 3];
    let l = k;
    // Get a slice of the first entry in the k array
    let m = &k[0..1];
    let n = m;
    println!("Arrays and array clices are copyable: {:?} {:?}", k, m);

    // Tuples too
    let o = (1, "str");
    let p = o;
    println!("Tuples are copyable {:?}", o);

    // That was the types that are copyable by default
    // We can also implement Copy on our own types as long
    // as they hold copyable types themselves
    struct Point {
        x: i32,
        y: i32,
    }
    impl Copy for Point {}
    impl Clone for Point { fn clone(&self) -> Point { *self } }
    let q = Point { x: 1, y: 10 };
    let r = q;
    println!("We made a copyable struct! {}/{}", q.x, q.y);

    // Phew! that was some black magic. Copy and Clone are `traits` that are built into language
    // Traits are basically interfaces that you can implement on your types
    // You can create them yourselves too, which we'll get to in a future post
    // There is an easier way to make a type copyable though:
    #[derive(Copy, Clone, Debug)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let s = Point3D { x: 1, y: -20, z: 30 };
    let t = s;
    println!("Point3D is now copyable, and the Debug trait let's us print it easily {:?}", s);
}
