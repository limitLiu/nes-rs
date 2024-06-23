use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(New)]
pub fn derive_new(i: TokenStream) -> TokenStream {
  fn generate(syntax_tree: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = syntax_tree.ident;

    let fields = if let Data::Struct(structure) = syntax_tree.data {
      structure.fields
    } else {
      return Ok(quote::quote! {
          compile_error!("new macro only supports structs");
      });
    };

    let field_params = fields.iter().map(|field| {
      if let Some(id) = &field.ident {
        let ty = &field.ty;
        quote::quote! { #id: #ty }
      } else {
        quote::quote! {
            compile_error!("Fields without identifiers are not supported");
        }
      }
    });

    let field_init = fields.iter().map(|field| {
      if let Some(id) = &field.ident {
        quote::quote! { #id }
      } else {
        quote::quote! {
            compile_error!("Fields without identifiers are not supported");
        }
      }
    });

    Ok(quote::quote! {
      impl #struct_name {
          pub fn new(#(#field_params),*) -> #struct_name {
              #struct_name { #(#field_init),* }
          }
      }
    })
  }
  let syntax_tree = parse_macro_input!(i as DeriveInput);
  match generate(syntax_tree) {
    Ok(st) => st.into(),
    Err(e) => e.to_compile_error().into(),
  }
}
