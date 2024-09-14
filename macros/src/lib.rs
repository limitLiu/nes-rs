use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemStruct};

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

#[proc_macro_attribute]
pub fn public(_attributes: TokenStream, i: TokenStream) -> TokenStream {
  fn generate(syntax_tree: ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let vis = syntax_tree.vis;
    let struct_name = syntax_tree.ident;

    let fields = match syntax_tree.fields {
      Fields::Named(fields) => fields.named,
      _ => {
        return Ok(quote::quote! {
            compile_error!("pub vis macro only supports structs");
        })
      }
    };

    let field_params = fields.iter().map(|field| {
      if let Some(id) = &field.ident {
        let ty = &field.ty;
        quote::quote! { pub #id: #ty }
      } else {
        quote::quote! {
            compile_error!("Fields without identifiers are not supported");
        }
      }
    });

    Ok(quote::quote! {
        #vis struct #struct_name {
          #(#field_params),*
        }
    })
  }
  let syntax_tree = parse_macro_input!(i as ItemStruct);
  match generate(syntax_tree) {
    Ok(st) => st.into(),
    Err(e) => e.to_compile_error().into(),
  }
}
