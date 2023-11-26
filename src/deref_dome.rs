


pub fn start() {
    // dome01();
    // dome02();
    // dome03();
    dome04();
}

// dome01
// fn dome01() {
//     let b = Box::new(5);
//     print!("b = {}", b);
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// dome02
// fn dome02() {
//     use crate::deref_dome::List::{Cons, Nil};
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// dome03
// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// impl<T> std::ops::Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
// fn dome03() {
//     // let x = 5;
//     // let y = &x;
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);

//     // let x = 5;
//     // let y = Box::new(x);
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);

//     // let x = 5;
//     // let y = MyBox::new(x);
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);
//     let n = MyBox::new(String::from("Rust"));
//     hello(&n);
//     fn hello(name: &str) {
//         println!("Hello, {}!", name);
//     }
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer  {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn dome04() {
    let a = CustomSmartPointer { data: String::from("my stuff") };
    let b = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    drop(a);
}