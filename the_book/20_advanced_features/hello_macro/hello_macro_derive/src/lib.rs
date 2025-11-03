use proc_macro::TokenStream;
use quote::quote;

// Procedual macro for `HelloMacro` trait
// (same for almost every procedual macro crate)
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Get an identifier
    let name = &ast.ident;

    // Generate Rust codes by using the identifier
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}

// Attribute-like macro `prologue`
#[proc_macro_attribute]
pub fn prologue(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse `item` as a function
    let item_ast = syn::parse_macro_input!(item as syn::ItemFn);

    impl_prologue_macro(&item_ast)
}

fn impl_prologue_macro(item_ast: &syn::ItemFn) -> TokenStream {
    let fname = &item_ast.sig.ident; // Function name
    let stmt = &item_ast.block.stmts[0]; // First statement in a block

    let generated = quote! {
        fn #fname() {
            println!("*** prologue ***");
            #stmt;
        }
    };
    generated.into()
}
