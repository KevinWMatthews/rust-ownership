#[derive(Copy, Clone)]
struct CopyType {
    data: i32,
}

#[allow(unused)]
fn main() {
    let copy_type = CopyType {
        data: 42,
    };

    let copy_type_ref = &copy_type;

    // Dereference and use
    let data = (*copy_type_ref).data;

    // Idiomatic Rust
    let data = copy_type_ref.data;

    // Derefernece and assign
    let copy_type_deref = *copy_type_ref;
}
