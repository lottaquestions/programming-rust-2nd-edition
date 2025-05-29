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
}
