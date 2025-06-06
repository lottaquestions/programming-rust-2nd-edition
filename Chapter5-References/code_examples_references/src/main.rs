use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn artist_table_creator() -> Table{
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madigrals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    table
}

fn sort_works(table: &mut Table){
    for (_artit, works) in table {
        works.sort();
    }
}
fn moving_scenario() {
    fn show(table: Table){
        for (artist, works) in table {
            println!("Works by {}:", artist);
            for work in works {
                println!("    {}", work);
            }
        }
    }
    let table = artist_table_creator();
    show(table);
}

fn reference_scenario(){
    fn show(table: &Table){
        // We do not have to explicitly dereference the reference using *
        // because the before for loop expands artist and works using
        // . (dot) notation, and values to the left of a dot are automatically
        // dereferenced in Rust.
        for (artist, works) in table {
            println!("Works by {}:", artist);
            for work in works {
                println!("    {}", work);
            }
        }
    }
    let mut table  = artist_table_creator();
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madigrals");
    sort_works(&mut table);
    assert_eq!(table["Gesualdo"][0], "Tenebrae Responsoria");

}

// Lifetimes and receiving references as function arguments
static WORTH_POINTING_AT : i32 = 1000; 

static  mut STASH: &i32 = &10;
// This function will only accept parameters with static lifetime.
fn static_stasher(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// Example showing explicit adding of lifetime
fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    moving_scenario();
    reference_scenario();

    // More reference examples
    let x = 10;
    let r = &x; // &x is a shared reference to x
    assert!(*r==10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
    assert!(*m == y);

    // In Rust, the . operator implicitly dereferences its left operand, if needed
    struct Anime { name: &'static str, bechdel_pass: bool }
    let aria = Anime{ name: "Aria: The Animation", bechdel_pass: true};
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!(anime_ref.bechdel_pass, true);

    // With explicit dereferencing, the equivalent would be:
    assert_eq!((*anime_ref).name,"Aria: The Animation");

    // dot (.) operator can also implicitly borrow a reference to its left operand
    let mut v = vec![1973, 1968];
    v.sort(); // implicitly borrows a mutable reference to v
    (&mut v).sort(); // Equivalent to the above but more verbose.
    assert_eq!(v[0], 1968);
    println!("{:?}", v);

    // References of references
    struct Point { x: i32, y: i32}
    let point = Point{x:1000, y:729};
    let r_point = &point;
    let rr_point = &r_point;
    let rrr_point = &rr_point;
    assert_eq!(rrr_point.x, 1000); // Rust automatically traversers 3 references to get to x
    assert_eq!(rrr_point.y, 729); // Rust automatically traversers 3 references to get to y

    // Comparing references
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert_eq!(rrx, rry);
    assert!( rrx == rry);             // The referents are equal
    assert!(!std::ptr::eq(rrx, rry)); // but occupy different addresses.

    fn factorial(n: usize) -> usize {
        (1..n+1).product()
    }
    let r = &factorial(6);
    // Arithmetic operators can see through one level of references
    assert_eq!(r + &1009, 1729);


    // This function expects a static lifetime for its parameter. 
    // See its definition
    static_stasher(&WORTH_POINTING_AT);

    {
        // Use scoping to ensure lifetime of s matches that of parabola
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    // How to define separate lifetimes for each member of a struct of references
    // Note that this is only for when the members are references.
    struct S<'a, 'b>{
        x: &'a i32,
        y: &'b i32

    }
    // By haaving x and y with different lifetimes, we are able to write the code below:
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y};
            r = s.x;
            assert_eq!(s.y, &y);
        }
        assert_eq!(r, &x);
    }

    // Omitting Lifetime Parameters
    struct  StringTable {
        elements: Vec<String>,
    }

    impl  StringTable {
        // In the method below, the Rust compiler deduces different lifetimes for 
        // self and prefix, even though they are not explicitly given by the programmer.
        fn find_by_prefix(&self, prefix: &str) -> Option<&String>{
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return  Some(&self.elements[i]);
                }
            }
            None
        }
    }
    let str_table = StringTable{ elements : vec!["random element in vec".to_string(), "not the random element".to_string()]};
    let my_prefix = "random".to_string();
    let found_elem = str_table.find_by_prefix(&my_prefix);
    assert_eq!(found_elem.unwrap(), &str_table.elements[0]);
}
