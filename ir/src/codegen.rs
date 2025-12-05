// use quote::quote;
// use proc_macro::TokenStream;
// use crate::ast::Game;

// pub fn print_ast_runtime(parsed: &Game) -> TokenStream {
//     // Generate Rust code that prints the AST when the program runs
//     let output = quote! {
//         fn main() {
//             println!("{:#?}", #parsed);
//         }
//     };
//     output.into()
// }
