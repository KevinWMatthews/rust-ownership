struct NonCopyType {
    data: i32,
}

#[allow(unused)]
fn main() {
    let non_copy_type = NonCopyType {
        data: 42,
    };

    let non_copy_type_ref = &non_copy_type;

    // Can dereference and use
    let data = non_copy_type_ref.data;

    // Can not dereference and assign
    // This takes ownership through a borrow,
    // but a borrow can't own something... it's borrowed.
    // Compiler error:
    // cannot move out of borrowed content
    // let non_copy_type_deref = *non_copy_type_ref;
}
