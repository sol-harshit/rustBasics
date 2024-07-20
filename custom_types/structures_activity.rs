/*
 * The below program solves the question 
 * Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
 */
#![allow(dead_code)]

#[derive(Debug)]
/// Represents a point in a two-dimensional space.
struct Point {
    x: f32, 
    y: f32,
}

#[derive(Debug)]
/// Represents a rectangle defined by the coordinates of its top-left and top-right points.
struct LengthPoints {
    top_left: Point, 
    top_right: Point,
}

#[derive(Debug)]
/// Represents a rectangle defined by its bottom-left and bottom-right points.
struct BreadthPoints {
    bottom_left: Point, 
    bottom_right: Point,
}

/// Represents a structure that holds the length and breadth points of an area.
struct AreaPoints {
    length: LengthPoints,
    breadth: BreadthPoints,
}

/// Represents the breadth points of a shape.
///
/// The `breadth_points` struct contains the bottom left and bottom right points of a shape.
/// These points define the breadth of the shape.
///
/// # Fields
///
/// * `bottom_left` - The bottom left point of the shape.
/// * `bottom_right` - The bottom right point of the shape.
///
/// # Example
///
/// ```
/// use custom_types::structures_activity::{BreadthPoints, Point};
///
/// let breadth_points: BreadthPoints = BreadthPoints {
///     bottom_left: Point { x: 0.4, y: 4.8 },
///     bottom_right: bottom_right,
/// };
/// ```

fn main() {
    let point: Point = Point{x: 0.4, y: 10.2};

    let bottom_right: Point = Point{x: 16.2, y: 4.8};

    let Point{x: left_edge, y: right_edge} = bottom_right;

    let length_points: LengthPoints = LengthPoints{top_left: point, top_right: Point{x: 16.2, y:10.2}};


    let LengthPoints{top_left: top_left_point, top_right: top_right_point} = length_points;

    let breadth_points: BreadthPoints = BreadthPoints{bottom_left: Point{x: 0.4, y:4.8}, bottom_right: bottom_right};

    let area = calculate_rectangle_area(top_left_point, top_right_point, breadth_points); 

    println!("Area of the rectangle is {}!", area);
}


/// Calculates the length and breadth of a shape using the given points.
///
/// # Arguments
///
/// * `top_left` - The top left point of the shape.
/// * `top_right` - The top right point of the shape.
/// * `breadth_points` - The points used to calculate the breadth of the shape.
///
/// # Returns
///
/// A tuple containing the calculated length and breadth of the shape.
///
fn calculate_rectangle_area(top_left: Point, top_right: Point, breadth_points: BreadthPoints) -> f32 {
    let (length, breadth) = calculate_length(LengthPoints{top_left: top_left, top_right: top_right}, breadth_points);
    length * breadth
}

fn calculate_length(length_points: LengthPoints, breadth_points: BreadthPoints) -> (f32, f32) {
    println!("Points are length_points: {:?}, breadth_points: {:?}", length_points, breadth_points);
    (length_points.top_right.x - length_points.top_left.x, length_points.top_right.y - breadth_points.bottom_right.y)
}