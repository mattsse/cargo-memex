pub mod build;
pub mod jpeg;
mod meme;
pub mod run;
mod workspace;

// /// Marks a function to be put in a meme.
// ///
// /// ```rust
// /// #[memex::main(meme = "../my-meme.jpg")]
// /// fn main() {
// ///     println!("Hello world");
// /// }
// /// ```
// #[proc_macro_attribute]
// pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
//     meme::main(args, item)
// }
