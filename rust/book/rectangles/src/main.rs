fn main() {
    // let height1 = 30;
    // let width1 = 50;

    // println!(
    //     "the area of the rectangle is {} square pixels",
    //     area(width1, height1)
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} square pixels",
        area(&rect1)
    );

    println!(
        "the area of the rectangle is {} square pixels",
        (rect1.area())
    );

    // this will throw an error -> Display is not implemented
    // println!("rect1 is {rect1}");

    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // dbg returns ownership of the expression's value
        height: 10,
    };

    dbg!(&rect2); // here we use the reference because we don't wait it to take the ownership
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// we want to borrow the struct, not keep the ownership
// so that main can contunie using it
fn area(rectangle: &Rectangle) -> u32 {
    // accessing fields of a burrowed struct does not move the field values!
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions
    // methods if have self as the first parameters
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // when they are not methods, usually they're constructors
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
