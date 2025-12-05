// use std::path::Path;

// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;

// use ir::fsm::*;
// use ast::ast::*;

// // ------------------------
// // Proc-macro entry point
// // ------------------------

// #[proc_macro]
// pub fn game(input: TokenStream) -> TokenStream {
//     // parse the input as Game
//     let parsed = syn::parse::<Game>(input);

//     // Print AST at compile time for debugging
//     // dbg!(&parsed);  // This will print to your build output

//     match parsed {
//         Ok(game) => {
//             // For now, the macro expands to a const expression which just contains debug info
//             // In a real-world use you would translate the AST to Rust code/structs or generate implementations.
//             let debug = format!("Parsed game with {} top-level flow components", game.flows.len());
//             let expanded = quote! {
//                 const _: &str = #debug;
//             };

//             let fsm = build_fsm(&game);
//             fsm_to_dot(&fsm);

//             expanded.into()

//         }
//         Err(err) => {
//             err.to_compile_error().into()
//         }
//     }
// }

// #[proc_macro]
// pub fn game2(input: TokenStream) -> TokenStream {
//     let game = syn::parse_macro_input!(input as Game);

//     // build the FSM
//     let fsm = build_fsm(&game);

//     // where to write?
//     let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
//     let out_path = Path::new(&manifest).join("generated_fsms");

//     std::fs::create_dir_all(&out_path).unwrap();

//     // filename is usually the game name, but here a default
//     let file_path = out_path.join("game_fsm.ron");

//     let serialized = ron::to_string(&fsm).unwrap();
//     std::fs::write(&file_path, serialized).unwrap();

//     // Now we return code that loads this file:
//     let tokens = quote! {
//         {
//             const DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated_fsms/game_fsm.ron"));
//             ron::from_str::<FSM>(DATA).unwrap()
//         }
//     };

//     tokens.into()
// }
