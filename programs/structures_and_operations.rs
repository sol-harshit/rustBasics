// Program demonstrate the creation and usage of structures 
// and operations on them.

// Define a structure named 'Point' with two fields 'x' and 'y'

struct Point {
    x: i32,
    y: i32,
}

// impl block is used to define methods on a structure and is similar to type and method in golang 
// type Point struct {
//     x, y float64
// }
// func (p Point) distance(q Point) float64 {
//     return math.Sqrt((p.x-q.x)*(p.x-q.x) + (p.y-q.y)*(p.y-q.y))
// }

impl Point {
    // This is a static method
    fn move_right(self, distance: i32) -> Point {
        Point { x: self.x + distance, y: self.y }
    }
}

fn main() {
    let start_point = Point { x: 0, y: 0 };
    
    let moved_point = start_point.move_right(10);
    println!("Moved point is at ({}, {})", moved_point.x, moved_point.y);

}