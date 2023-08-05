#![allow(unused)]

fn main() {
    // the data is stored on the heap.
    // the variable `original` is a pointer that lives on the stack and points to the data that is on the heap.
    let original = String::from("original value");
    println!("original: \t\"{}\"", original);

    //// by setting `next` equal to `original` we are giving the ownership of the data
    //// from `original` to `next`, so basically `original` no longer exists from this point on.
    // let next = original;
    // println!("{}", original); // compile-time error here

    // this copies the data and given `next` the ownership of the next data.
    // now we have duplicated data on the heap, but we can use both variables
    let next = original.clone();
    println!("original: \t\"{}\"", original); // no compile-time error here

    // this syntax means `next` is temporary borrowing the memory that `original` points to
    let next = &original;
    // but this still works, because so far everything is immutable, so the value in memory cannot change
    println!("original: \t\"{}\"", original);

    //// let's see what happens if the value is mutable

    let mut original = String::from("original value");
    println!("original: \t\"{}\"", original);

    let next = &original;
    // this still works, because the compiler can see that even though the value in memory
    // is mutable and can change, it is not changing
    println!("original: \t\"{}\"", original);

    //// let's try to change the value

    let mut original = String::from("original value");
    println!("original: \t\"{}\"", original);
    original = String::from("new value");

    let next = &original;
    // this still works, because nothing is changing after the borrowing takes place
    println!("original: \t\"{}\"", original);

    //// one more try
    let mut original = String::from("original value");
    println!("original: \t\"{}\"", original);

    //// now we have a compile-time error, because once borrowing occurs Rust cannot guarantee memory safety
    //// we cannot change the data once we have read-only references of that data downstream
    // let next = &original;
    // original = String::from("new value");
    // println!("original: \t\"{}\"", next);

    // a variable returns the ownership of memory once it gets out of scope
    let mut original = String::from("original value");
    println!("Outer original value: \t{}", original);

    {
        // the `mut` keyword means that `next` can change the data
        // without it, it's just read-only
        let next = &mut original;
        *next = String::from("next value");

        println!("Inner next: \t{}", next);
        println!("Inner original: \t{}", original);
    } // here `next` goes out of scope and ownership returns to `original`

    // here `original` has changed to `next value`
    println!("Outer original value: \t{}", original);

    //// Lifetime

    // let outer_scope;
    // {
    //     let inner_scope = 5;
    //     // compile-time error here, because `inner_scope` does not live long enough,
    //     // to be used outside of the scope
    //     outer_scope = &inner_scope;
    // }
    // println!("{}", outer_scope)

    // the compiler is fine with that, because it sees that the scope
    // that is sending the reference, is the same one that is getting the result back
    let value = 6;
    let param = return_one_parameter(&value);
    println!("{}", param);

    let value_one = 5;
    let value_two = 6;
    let param = explicit_lifetime(&value_one, &value_two);
}

// // compile-time error here, because the variable `value` will be used beyond its scope
// fn return_bad_red() -> &i32 {
//     let value = 5;
//     return &value;
// }

fn return_one_parameter(i: &i32) -> &i32 {
    return i;
}

// // compile-time error here, because the compiler does not know which parameter lifetime to use
// fn explicit_lifetime(p1: &i32, p2: &i32) -> &i32 {
//     if p1 > p2 {
//         return p1;
//     }
//     return p2;
// }

// tell the compiler that both parameters have the same lifetime
// and also that the return value has the same lifetime
fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        return p1;
    }
    return p2;
}
