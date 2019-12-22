#![feature(destructuring_assignment)]

<<<<<<< HEAD
const C: i32 = 1;

=======
>>>>>>> Implement destructuring assignment (squashed)
fn main() {
    let (mut a, mut b);
    (a, .., b, ..) = (0, 1); //~ ERROR `..` can only be used once per tuple pattern
    (a, a, b) = (1, 2); //~ ERROR mismatched types
<<<<<<< HEAD
    (C, ..) = (0,1); //~ ERROR invalid left-hand side of assignment
=======
>>>>>>> Implement destructuring assignment (squashed)
    (_,) = (1, 2); //~ ERROR mismatched types
}
