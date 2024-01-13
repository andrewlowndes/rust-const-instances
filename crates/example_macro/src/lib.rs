use to_expr_core::ToExpr;
use to_expr_example_lib::{MySimpleConstObj, MyTestStruct};

#[proc_macro]
pub fn example_expr(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //we can contruct any objects here and then convert them to expression code to be used directly in the response
    let my_instance = MyTestStruct {
        prop1: "test".to_string(),
        prop2: 23,
        prop3: None,
        prop4: Some("other".to_string()),
    };

    my_instance.to_expr().into()
}

#[proc_macro]
pub fn example_const_expr(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //we will create an object here that will be used in a constant context
    let test_str = (0..10).map(|num| num.to_string()).collect::<String>();
    format!("My {test_str} test string");

    let my_instance = MySimpleConstObj { my_val: &test_str };

    my_instance.to_expr().into()
}
