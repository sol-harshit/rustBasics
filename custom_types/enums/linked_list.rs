// we will use enums to create a linked list 
// we will use Box to create a recursive data structure

enum List {
    // A tuple struct is a struct where the fields do not have names, they are just types. 
    // Box is a type of smart pointer in Rust that allows for heap allocation. 
    // This is necessary because Rust needs to know the size of types at compile time. If we had Cons(i32, List), 
    // then our List type would have an infinite size because List contains itself.
    Cons(i32, Box<List>), // tuple struct that wraps an element and a pointer to the next node, Box is a smart pointer
    Nil, // a node that signifies the end of the list
}


// methods can be attached to an enum, think of it like method receivers in Go
impl List {
    fn new() -> List {
        List::Nil // since first node will be empty, we return Nil
    }

    fn prepend(self, elem: i32) -> List {
        // `Cons` also has type `List`
        List::Cons(elem, Box::new(self)) // prepend an element to the list
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // can't take ownership of the tail, because `self` is borrowed, instead take a reference to the tail hence the use of `ref`
            List::Cons(_, ref tail) => 1 + tail.len(), // if Cons, increment the length by 1 and call len on the tail
            List::Nil => 0, // if Nil, return 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self { 
            List::Cons(head, ref tail) => {
                // format! is similar to println! but returns a heap allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify()) // if Cons, format the head and call stringify on the tail
            },
            List::Nil => {
                format!("Nil") // if Nil, return Nil
            },
        }
    }
}

fn main() {
    let mut list = List::new(); // create a new list
    list = list.prepend(1); // prepend 1 to the list
    list = list.prepend(2); // prepend 2 to the list
    list = list.prepend(3); // prepend 3 to the list

    println!("linked list has length: {}", list.len()); // print the length of the list
    println!("{}", list.stringify()); // print the list
}