struct Point<T> {
    x: T,
    y: T 
}

struct Point2<T, V> {
    x: T,
    y: V
}

struct Line1<T> {
    start: Point<T>,
    end: Point<T>
}

struct Line2<T, V> {
    start: Point2<T, V>,
    end: Point2<T, V>
}

fn generics() {
    let a:Point<u16> = Point { x: 0, y: 0};
    let a2:Point<u16> = Point { x: 1, y: 2};
    let myline = Line1 { start: a, end: a2};

    let b:Point2<f32, f64> = Point2 { x: 1.2, y: 3.4};
    let b2:Point2<f32, f64> = Point2 { x: 4.2, y: 2.4};
    let myline2 = Line2 { start: b, end: b2};
}

fn main() {
    generics();
}
