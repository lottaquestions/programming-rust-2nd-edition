
fn print_padovan(){
    let mut padovan = vec![1,1,1]; // allocated here
    for i in 3..10{
        let next = padovan[i - 3] + padovan [i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn test_struct_ownership(){
    struct Person {
        name: String,
        birth: i32
    }
    let mut composers = Vec::new();
    composers.push(Person {name: "Palestrina".to_string(), birth: 1525});
    composers.push(Person { name: "Dowland".to_string(), birth: 1563});  
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth );
    }
}
// padovan vec dropped here
fn main() {
    // =========
    // Ownership
    // =========
    print_padovan();

    {
        // When the program calls Box::new, it allocates space 
        // for a tuple of two f64 values on the heap, moves its
        // argument (0.625, 0.5) into that space, and returns 
        // a pointer to it
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both point and label dropped here

    test_struct_ownership();

    // =========
    // Moves
    // =========
    // string literals are normally put in read-only memory, so to create heap allocated strings,
    // we use to_string()
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    // let u = s; // this cannot work as s was already moved to t previously
    println!("{:?}", t);

    // To obtain copies of an object during assignment, use clone, which performs a deep copy
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();
    assert_eq!(s, t);
    assert_eq!(s, u);

    // =========================
    // More Operations That Move
    // =========================
    let mut s = "Govinda".to_string();
    assert_eq!(s, "Govinda");
    s = "Siddartha".to_string(); // value of Govinda dropped here
    assert_eq!(s, "Siddartha");

    let mut s = "Govinda".to_string();
    let t = s;
    assert_eq!(t, "Govinda");
    s = "Siddartha".to_string(); // nothing is dropped here
    assert_eq!(s, "Siddartha"); 

    // In general: Passing arguments to functions moves ownership to the function’s parameters; 
    // returning a value from a function moves ownership to the caller. Building a tuple moves
    // the values into the tuple. And so on.

    // Individual elements are normally not just moved out of vectors through assignment.
    // This would leave the vector in an incomplete state, and state information would
    // have to be stored concerning which elements are present in the vector and which ones are
    // not. Instead, references of elements are normally taken. For example

    // Build a vector of the strings "101", "102", ..., "105"
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let third = &v[2]; // Get a ref to the 3rd element
    assert_eq!(third, "103");
    let fifth = &v[4];
    assert_eq!(fifth, "105"); // Get a ref to the 5th element

    // If one really wants to move the elements out of the vector, other options exist.
    // 1. Pop a value off of the end of the vector.
    let fifth = v.pop().expect("Vector is empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector, and move the last value into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    assert_eq!(&v[1], "104");

    // 3. Swap in another value for the one we are taking out
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    assert_eq!(&v[2], "substitute");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);

    let v = vec!["liberté".to_string(), "egalité".to_string(), "fraternité".to_string()];
    for mut s in v { // the vector is moved out of v, leaving v uninitialized.
        s.push('!'); // the for loop's internal machinery takes ownership. With each iteration,
                        // an element is moved into the variable s. s is then able to change
                        // the contents of the element inside of the loop
        println!("{}", s);
    }

    // The Option type is useful for scenarios where the compiler cannot keep track of moved values, e.g.
    // values inside of vectors.
    struct Person {name: Option<String>, birth: i32 }
    let mut composers = Vec::new();
    composers.push(Person{name: Some("Palestrina".to_string()), birth: 1525});

    // How to consume the name field in the composers vector:
    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
    assert_eq!(composers[0].birth, 1525);

    let mut composers = Vec::new();
    composers.push(Person{name: Some("Palestrina".to_string()), birth: 1525});

    // Another way to consume the name field in the composers vector using take:
    let first_name = composers[0].name.take(); // take will swap out the name with the default of value.
                                                               // The default value is None since name is of type Option
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
    assert_eq!(composers[0].birth, 1525);


}
