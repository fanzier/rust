#[macro_use]
mod underscore;

fn main() {
    underscore!();
<<<<<<< HEAD
    //~^ ERROR `_` can only be used on the left-hand side of an assignment
    //~| ERROR destructuring assignments are unstable
=======
    //~^ ERROR expected expression, found reserved identifier `_`
    //~^^ ERROR destructuring assignments are unstable
>>>>>>> Implement destructuring assignment (squashed)
}
