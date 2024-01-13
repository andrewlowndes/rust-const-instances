use to_expr_macro::ToExpr;

//example struct we will use in the `example` and `example_macro` crates
#[derive(Debug, ToExpr)]
pub struct MyTestStruct {
    pub prop1: String,
    pub prop2: u32,
    pub prop3: Option<Box<MyTestStruct>>,
    pub prop4: Option<String>,
}

#[derive(Debug, ToExpr)]
pub struct MySimpleConstObj<'a> {
    pub my_val: &'a str,
}
