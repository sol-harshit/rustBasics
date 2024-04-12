// enums are types which have a few definite values
// The reason you don't need to define structs differently when they are part of an enum in Rust is due to the way Rust's type system is designed. 
// Each variant of an enum can directly incorporate data, and the syntax for doing so is integrated into the enum itself. 

// define an enum called WebEvent
enum WebEvent {
    // An enum may be either unit-like
    PageLoad, // we don't need to define a struct here explicitly
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or like structures
    Click { x: i64, y: i64 },
}
