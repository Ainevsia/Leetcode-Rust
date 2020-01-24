const GLOBAL: i32 = 123;
static mut s: i32 = 1;
const X: My = My { x: 1, };

fn main() {
    let local = 123124;
    println!("local\t = {:p}", &local);
    // println!("global = {:p}", &GLOBAL);
    let mut x = Some(123);
    println!("x\t = {:p}", &x);
    let y = x.take();
    println!("x = {:#?}", x);
    println!("x\t = {:p}", &x);
    println!("y\t = {:p}", &y);

    let mut stack = vec![];
    println!("stack = {:#?}", stack);
    println!("stack\t = {:p}", &stack);
    stack.push(123);
    println!("stack = {:#?}", stack);
    println!("stack\t = {:p}", &stack);
    let x = stack.pop();
    println!("x = {:#?}", x);
    println!("x\t = {:p}", &x);
    println!("s = {:#?}", s);
    s = 2;
    println!("s = {:#?}", s);
    
    
}


struct My {
    x: i32,
}

