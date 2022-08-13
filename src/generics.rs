
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also specify constraints on generic types when defining methods on the type. We could,
// for example, implement methods only on Point<f32> instances rather than on Point<T> instances
// with any generic type. In Listing 10-10 we use the concrete type f32, meaning we don’t declare
// any types after impl.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic type parameters in a struct definition aren’t always the same as those you use in that
// same struct’s method signatures. Listing 10-11 uses the generic types X1 and Y1 for the Point
// struct and X2 Y2 for the mixup method signature to make the example clearer. The method creates
// a new Point instance with the x value from the self Point (of type X1) and the y value from the
// passed-in Point (of type Y2).
struct MultiPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MultiPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MultiPoint<X2, Y2>) -> MultiPoint<X1, Y2> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());


    //--------------------
    let p1 = MultiPoint { x: 5, y: 10.4 };
    let p2 = MultiPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The largest_i32 function is the one we extracted in Listing 10-3 that finds the largest i32 in
// a slice. The largest_char function finds the largest char in a slice. The function bodies have
// the same code, so let’s eliminate the duplication by introducing a generic type parameter in a
// single function.

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}