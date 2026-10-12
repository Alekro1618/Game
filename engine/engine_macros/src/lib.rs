

use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, token::Token, *};
use quote::{format_ident, quote};
use proc_macro_crate::crate_name;

extern crate proc_macro;

fn engine_path() -> proc_macro2::TokenStream {
    match crate_name("engine").expect("Couldn't found `engine crate`") {
        proc_macro_crate::FoundCrate::Itself => quote!(crate),
        proc_macro_crate::FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote!(::#ident)
        },
    }
}

#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #engine_path::ecs::components::Component
            for #name #ty_generics #where_clause {
            const STORAGE_TYPE: #engine_path::ecs::components::StorageType = #engine_path::ecs::components::StorageType::Table;

            type Mutability = #engine_path::ecs::components::Mutable;
        }
    }
    .into()
}

#[proc_macro_derive(Resource, attributes(component))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #engine_path::ecs::components::Component
            for #name #ty_generics #where_clause {
            const STORAGE_TYPE: #engine_path::ecs::components::StorageType = #engine_path::ecs::components::StorageType::Table;

            type Mutability = #engine_path::ecs::components::Mutable;
        }

        impl #impl_generics #engine_path::ecs::resource::Resource
            for #name #ty_generics #where_clause {
        }
    }
    .into()
}

#[proc_macro_derive(Message, attributes(component))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    quote! {
        impl #impl_generics #engine_path::ecs::message::Message
            for #name #ty_generics #where_clause {
        }
    }
    .into()
}

#[proc_macro_derive(Event, attributes(component))]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #engine_path::ecs::event::Event
            for #name #ty_generics #where_clause {
        }
    }
    .into()
}


#[proc_macro_derive(Asset, attributes(component))]
pub fn derive_asset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #engine_path::asset::asset::Asset
            for #name #ty_generics #where_clause {
        }
    }
    .into()
}


#[proc_macro_derive(Bundle, attributes(component))]
pub fn derive_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Bundle can only be derived for structs with named fields"),
        },
        _ => panic!("Bundle can only be derived for structs"),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    
    let type_param_idents : Vec<_> = input
        .generics
        .type_params()
        .map(|tp| &tp.ident)
        .collect();

    let expanded = quote! {
        impl #impl_generics #engine_path::ecs::bundle::Bundle 
            for #name #ty_generics #where_clause {
        
            fn component_ids(
                components: &mut #engine_path::ecs::components::register::ComponentsRegistrator,
            ) -> impl Iterator<Item = (std::any::TypeId, #engine_path::ecs::components::id::ComponentId)> + use<#(#type_param_idents),*> {
                <(#(#field_types),*) as #engine_path::ecs::bundle::Bundle>::component_ids(components)
            }

            fn insert_into_table(self, components : &#engine_path::ecs::components::id::ComponentIdsArray, table: &mut #engine_path::ecs::storage::Table, tick : #engine_path::ecs::change_detection::tick::Tick) {
                let #name { #(#field_names),* } = self;
                (#(#field_names,)*).insert_into_table(components, table, tick);
            }

            fn replace_in_table(self, components : &#engine_path::ecs::components::id::ComponentIdsArray, table: &mut #engine_path::ecs::storage::Table, row : usize, tick : #engine_path::ecs::change_detection::tick::Tick) {
                let #name { #(#field_names),* } = self;
                (#(#field_names,)*).replace_in_table(components, table, row, tick);
            }

            fn remove_from_table(components : &#engine_path::ecs::components::id::ComponentIdsArray, table: &mut #engine_path::ecs::storage::Table, row : usize, tick : #engine_path::ecs::change_detection::tick::Tick) {
                <(#(#field_types,)*) as #engine_path::ecs::bundle::Bundle>::remove_from_table(components, table, row, tick);
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let engine_path  = engine_path();

    let mut generics = input.generics.clone();
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(#engine_path::ecs::::reflect::Reflect));
            type_param.bounds.push(parse_quote!(Clone));
            type_param.bounds.push(parse_quote!(Send));
            type_param.bounds.push(parse_quote!(Sync));
            type_param.bounds.push(parse_quote!('static));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut where_clause = where_clause.cloned().unwrap_or_else(|| parse_quote! { where });
    where_clause.predicates.push(parse_quote! { #name #ty_generics: Clone });

    let expanded = quote! {
        impl #impl_generics #engine_path::ecs::reflect::Reflect for #name #ty_generics #where_clause {
            fn as_reflect(&self) -> &dyn #engine_path::ecs::reflect::Reflect {
                self
            }

            fn into_reflect(self) -> Box<dyn #engine_path::ecs::reflect::Reflect> {
                Box::new(self)
            }

            fn reflect_clone(&self) -> Result<Box<dyn #engine_path::ecs::reflect::Reflect>, #engine_path::error::error::Error> {
                Ok(Box::new(Clone::clone(self)))
            }

            fn name(&self) -> String {
                String::from(::std::any::type_name::<#name>())
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn get_type_id(&self) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Self>())
            }
        }

        impl #impl_generics #engine_path::ecs::reflect::FromReflect for #name #ty_generics #where_clause {
            fn from_reflect(reflect: &dyn #engine_path::ecs::reflect::Reflect) -> Option<Self> {
                reflect.downcast_ref().cloned()
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_attribute]
pub fn reflect(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let traits = parse_macro_input!(attr with Punctuated::<Ident, Token![,]>::parse_terminated);
    let engine_path  = engine_path();
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // For each `Serialize` -> build the *real* ident `ReflectSerialize`
    let inserts = traits.iter().map(|t| {
        let reflect_ident = format_ident!("Reflect{}", t);
        quote! {
            type_register.insert::<#engine_path::ecs::type_register::#reflect_ident>(#engine_path::ecs::type_register::FromType::<Self>::from_type());
        }
    });

    let expanded = quote! {
        #input

        impl #impl_generics #engine_path::ecs::type_register::GetTypeRegistration for #name #ty_generics #where_clause {
            fn get_type_registration() -> #engine_path::ecs::type_register::TypeRegistration {
                let mut type_register = #engine_path::ecs::type_register::TypeRegistration::of::<Self>();
                type_register.insert::<#engine_path::ecs::type_register::ReflectFromReflect>(#engine_path::ecs::type_register::FromType::<Self>::from_type());
                #(#inserts)*
                type_register
            }
        }
    };

    expanded.into()
}