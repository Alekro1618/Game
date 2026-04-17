use proc_macro::TokenStream;
use syn::*;
use quote::quote;

extern crate proc_macro;

#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics engine::ecs::components::Component
            for #name #ty_generics #where_clause {
            const STORAGE_TYPE: engine::ecs::components::StorageType = StorageType::Table;

            type Mutability = Mutable;
        }
    }
    .into()
}

#[proc_macro_derive(Resource, attributes(component))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics engine::ecs::components::Component
            for #name #ty_generics #where_clause {
            const STORAGE_TYPE: engine::ecs::components::StorageType = StorageType::Table;

            type Mutability = Mutable;
        }

        impl #impl_generics engine::ecs::resource::Resource
            for #name #ty_generics #where_clause {
        }
    }
    .into()
}
