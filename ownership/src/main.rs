fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[derive(Clone, Copy)]
struct Rectangle{
    length: f64,
    width: f64,
}

fn perimeter(rect : Rectangle) -> f64{
    2.0 * (rect.length + rect.width)
}

fn perimeter2(rect :&Rectangle) -> f64{
    2.0 * (rect.length + rect.width)
}

fn main() {
    let x = 10.0;
    let y = 20.0;
    let my_rectangle = Rectangle { length: 5.0, width: 3.0 };


    let result = average(x, y);
    println!("La moyenne de {} et {} est : {}", x, y, result);

    let result2: f64 = average(x, y);
    println!("La moyenne de {} et {} est : {}", x, y, result2);

    let perimeter_result = perimeter2(&my_rectangle);
    println!("Le périmètre du rectangle est : {}", perimeter_result);

    let perimeter_result2 = perimeter2(&my_rectangle);
    println!("Le périmètre du rectangle est : {}", perimeter_result2);

    let clone1_result = perimeter(my_rectangle.clone());
    println!("Le périmètre du clone 1 est : {}", clone1_result);

    let clone2_result = perimeter(my_rectangle.clone());
    println!("Le périmètre du clone 2 est : {}", clone2_result);
}
