/*
    rustc
*/

// Line Comment
//! Doc Comment

/*! This is a testing function  */
fn testing_function() {
    let name = "foo";
    let char_literal = 's';
    let string_literal = "bar";
    let exponent = 12e+10;
    let float_exponent = 19.12e-7;
    let int = 10;
    let float = 1.45454;
}

pub struct Hey<'lifetime> {
    baz: &'lifetime str,
}

