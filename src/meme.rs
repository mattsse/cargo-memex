use std::borrow::Cow;
use std::str::FromStr;

pub static TRADE_OFFER: &[u8] = include_bytes!("../resources/trade-offer.jpg");

pub static BORROW_CHECKER: &[u8] = include_bytes!("../resources/borrow-checker.jpg");

pub static RUST_EXPERT: &[u8] = include_bytes!("../resources/rust-expert.jpg");

#[derive(Debug, Clone)]
pub struct MemeFile {
    pub name: String,
    pub content: Cow<'static, [u8]>,
}

impl FromStr for MemeFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
// #[derive(Default)]
// struct Config {
//     meme: Option<String>
// }
//
// impl Config {
//
//     fn set_meme(
//         &mut self,
//         meme: syn::Lit,
//         span: Span,
//     ) -> Result<(), syn::Error> {
//         let meme = parse_string(meme, span, "meme")?;
//
//         Ok(())
//     }
//
// }
//
// fn parse_config( args: &syn::AttributeArgs,) -> Result<Config, syn::Error> {
//     let mut config = Config::default();
//
//     for arg in args {
//         match arg {
//             syn::NestedMeta::Meta(syn::Meta::NameValue(namevalue)) => {
//                 let ident = namevalue.path.get_ident();
//                 if ident.is_none() {
//                     let msg = "Must have specified ident";
//                     return Err(syn::Error::new_spanned(namevalue, msg));
//                 }
//                 match ident.unwrap().to_string().to_lowercase().as_str() {
//                     "meme" => {
//
//                         config.set_meme(
//                             namevalue.lit.clone(),
//                             syn::spanned::Spanned::span(&namevalue.lit),
//                         )?;
//                     }
//                     name => {
//                         let msg = format!(
//                             "Unknown attribute {} is specified; expected one of: `meme`",
//                             name,
//                         );
//                         return Err(syn::Error::new_spanned(namevalue, msg));
//                     }
//                 }
//             }
//             syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
//                 let ident = path.get_ident();
//                 if ident.is_none() {
//                     let msg = "Must have specified ident";
//                     return Err(syn::Error::new_spanned(path, msg));
//                 }
//                 let name = ident.unwrap().to_string().to_lowercase();
//                 let msg = match name.as_str() {
//                     "meme" => {
//                         format!("The `{}` attribute requires an argument.", name)
//
//                     }
//                     name => {
//                         format!("Unknown attribute {} is specified; expected one of: `meme`", name)
//                     }
//                 };
//                 return Err(syn::Error::new_spanned(path, msg));
//             }
//             other => {
//                 return Err(syn::Error::new_spanned(
//                     other,
//                     "Unknown attribute inside the macro",
//                 ));
//             }
//         }
//     }
//
//     Ok(config)
// }
//
// fn parse_string(lit: syn::Lit, span: Span, field: &str) -> Result<String, syn::Error> {
//     match lit {
//         syn::Lit::Verbatim(lit) => Ok(lit.to_string()),
//         syn::Lit::Str(lit) => Ok(lit.value()),
//         _ => Err(syn::Error::new(
//             span,
//             format!("Failed to parse value of lit `{}` as string.", field),
//         )),
//     }
// }
//
// pub(crate) fn main(args: TokenStream, item: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(item as syn::ItemFn);
//     let args = syn::parse_macro_input!(args as syn::AttributeArgs);
//
//     if input.sig.ident == "main" && !input.sig.inputs.is_empty() {
//         let msg = "the main function cannot accept arguments";
//         return syn::Error::new_spanned(&input.sig.ident, msg)
//             .to_compile_error()
//             .into();
//     }
//
//
//     TokenStream::new()
// }
