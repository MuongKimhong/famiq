extern crate proc_macro;

use proc_macro::TokenStream;
use syn::*;
use quote::*;

#[proc_macro_attribute]
pub fn set_widget_attributes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    // Insert attributes field
    if let Fields::Named(fields) = &mut input.fields {
        fields.named.push(syn::parse_quote! {
            pub attributes: WidgetAttributes
        });
        fields.named.push(syn::parse_quote! {
            pub cloned_attrs: WidgetAttributes
        });
    }
    let expanded = quote! {
        #input

        impl SetWidgetAttributes for #name {
            fn attributes(&mut self) -> &mut WidgetAttributes {
                &mut self.attributes
            }

            fn cloned_attrs(&mut self) -> &mut WidgetAttributes {
                &mut self.cloned_attrs
            }
        }
    };
    expanded.into()
}
