use to_expr_example_lib::{MySimpleConstObj, MyTestStruct};
use to_expr_example_macro::{example_const_expr, example_expr};

const MY_CONST_OBJ: MySimpleConstObj<'static> = example_const_expr!();

fn main() {
    dbg!(&MY_CONST_OBJ);

    //say we have an instance of a struct, we want a function that takes an instance and transforms it into a serialised form
    let my_instance = example_expr!();
    dbg!(my_instance);
}
