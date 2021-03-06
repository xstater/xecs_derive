use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Component)]
pub fn component_derive(input : TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_component_macro(&ast)
}

fn impl_component_macro(ast : &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {}
    };
    gen.into()
}

#[proc_macro_derive(Resource)]
pub fn resource_derive(input : TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_resource_macro(&ast)
}

fn impl_resource_macro(ast : &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Resource for #name {}
    };
    gen.into()
}
