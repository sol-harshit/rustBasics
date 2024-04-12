#![allow(dead_code)]

enum Status {
    Rich, // as we have seen in the intro these are the unit like structs
    Poor, // so we don't need to define a struct here explicitly
}

enum Work {
    Civilian, 
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are aviailable without manual scoping
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `work`
    use crate::Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor; 

    // Equivalent to `Work::Civilian`
    let work = Civilian; 

    match status {
        // Note the lock of scoping because of the explicit `use` above. 
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money...."),
    }

    match work {
        // Note again the lack of scoping. 
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

}

