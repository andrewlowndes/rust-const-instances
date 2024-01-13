use proc_macro2::TokenStream;

pub trait ToExpr {
    fn to_expr(&self) -> TokenStream;
}

macro_rules! quote_self {
    ($($name:ty),*) => {
        $(
            impl ToExpr for $name {
                fn to_expr(&self) -> TokenStream {
                    ::quote::quote!(#self)
                }
            }
        )*
    };
}

quote_self!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, &str, bool
);

impl<T: ToExpr> ToExpr for Option<T> {
    fn to_expr(&self) -> TokenStream {
        match self {
            Some(val) => {
                let ident = val.to_expr();
                quote::quote!(Some(#ident))
            }
            None => quote::quote!(None),
        }
    }
}

impl<T: ToExpr, const N: usize> ToExpr for [T; N] {
    fn to_expr(&self) -> TokenStream {
        let items = self.iter().map(|item| item.to_expr());
        quote::quote!([(#(#items),*)])
    }
}

impl<T: ToExpr> ToExpr for Vec<T> {
    fn to_expr(&self) -> TokenStream {
        let items = self.iter().map(|item| item.to_expr());
        quote::quote!(vec![(#(#items),*)])
    }
}

impl<T: ToExpr> ToExpr for &[T] {
    fn to_expr(&self) -> TokenStream {
        let items = self.iter().map(|item| item.to_expr());
        quote::quote!(&[(#(#items),*)])
    }
}

macro_rules! quote_from {
    ($($name:ty),*) => {
        $(
            impl ToExpr for $name {
                fn to_expr(&self) -> TokenStream {
                    quote::quote!($name::from(#self))
                }
            }
        )*
    };
}

quote_from!(String);

macro_rules! quote_new {
    ($($name:path),*) => {
        $(
            impl<T: ToExpr> ToExpr for $name {
                fn to_expr(&self) -> TokenStream {
                    let ident = T::to_expr(self.as_ref());
                    quote::quote!(<$name>::new(#ident))
                }
            }
        )*
    };
}

quote_new!(Box<T>, ::std::sync::Arc<T>, ::std::rc::Rc<T>);

impl<T: ToExpr> ToExpr for ::std::cell::RefCell<T> {
    fn to_expr(&self) -> TokenStream {
        let ident = self.borrow().to_expr();
        quote::quote!(<$name>::new(#ident))
    }
}

impl<T: ToExpr> ToExpr for ::std::collections::HashSet<T> {
    fn to_expr(&self) -> TokenStream {
        let items = self.iter().map(|item| item.to_expr());
        quote::quote!(HashSet::from([(#(#items),*)]))
    }
}

impl<K: ToExpr, V: ToExpr> ToExpr for ::std::collections::HashMap<K, V> {
    fn to_expr(&self) -> TokenStream {
        let items = self.iter().map(|(key, val)| {
            let key = key.to_expr();
            let val = val.to_expr();
            quote::quote!((#key, #val))
        });
        quote::quote!(HashMap::from([(#(#items),*)]))
    }
}

impl ToExpr for () {
    fn to_expr(&self) -> TokenStream {
        quote::quote!(())
    }
}

impl<A: ToExpr> ToExpr for (A,) {
    fn to_expr(&self) -> TokenStream {
        let a = self.0.to_expr();
        quote::quote!((#a,))
    }
}

impl<A: ToExpr, B: ToExpr> ToExpr for (A, B) {
    fn to_expr(&self) -> TokenStream {
        let a = self.0.to_expr();
        let b = self.1.to_expr();
        quote::quote!((#a,#b))
    }
}

impl<A: ToExpr, B: ToExpr, C: ToExpr> ToExpr for (A, B, C) {
    fn to_expr(&self) -> TokenStream {
        let a = self.0.to_expr();
        let b = self.1.to_expr();
        let c = self.2.to_expr();
        quote::quote!((#a,#b,#c))
    }
}
