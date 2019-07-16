fn main() {
    example1();
    example2();
    example3();
    example4();
}

fn example1() {
    let x = 5;

    let y = &x; // <-- Lifetime of reference 'y' starts

    // Use the reference
    let _val = *y; // <-- Lifetime of reference 'y' ends

    // Do other stuff
    // ...
}

fn example2() {
    let x = 5;

    let y = &x; // <-- Lifetime of reference 'y' starts

    // Use the reference
    let _val = *y;

    // Do other stuff
    // ...

    // Use the reference again
    let _val2 = *y; // <-- Lifetime of reference 'y' ends
}

fn example3() {
    let mut x = 5;

    let y = &x; // <--Lifetime of reference 'y' starts

    // Use the reference
    let _val = *y; // <-- Lifetime of reference 'y' ends

    // The compiler determines that 'y' is never used again,
    // so it considers 'y' to be out of scope.
    // We can safely borrow mutably.
    let z = &mut x;

    // Use the reference
    let _val = *z; // <-- Lifetime of reference 'z' ends
}

fn example4() {
    let mut x = 5;

    let y = &x; // <-- Lifetime of reference 'y' starts

    // Use the reference
    let _val = *y; // <-- Lifetime of reference 'y' ends

    let z = &mut x;
    let _val = *z; // <-- Lifetime of reference 'z' ends

    // Create a new reference
    let y = &x; // <-- Lifetime of reference 'y' starts
    let _val = *y; // <-- Lifetime of new reference 'y' ends

    // Do not use 'z' again! This would extend its scope.
}
