// enums are types which have a few definite values
// The reason you don't need to define structs differently when they are part of an enum in Rust is due to the way Rust's type system is designed. 
// Each variant of an enum can directly incorporate data, and the syntax for doing so is integrated into the enum itself. 

// define an enum called WebEvent
enum WebEvent {
    // An enum may be either unit-like
    PageLoad, // we don't need to define a struct here explicitly
    PageUnload,
    Paste(String),
    // or like structures
    Click { x: i64, y: i64 },
    // like tuple structs
    KeyPress(char, i32),
}

// a function which takes WebEvent enum as an argument amd 
// returns nothing

fn inspect(event: WebEvent) { 
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c, d) => println!("Key pressed '{}' and number of times {}", c, d),
        WebEvent::Paste(s) => println!("Pasted '{}' ", s),
        WebEvent::Click {x, y} => {
            println!("Click at x={}, y={} ", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x', 0);
    // `to_owned()` creates an owned `String` from a string slice 
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y: 90};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

}
