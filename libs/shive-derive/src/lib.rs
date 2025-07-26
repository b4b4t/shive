extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, GenericArgument, PathArguments, Type};

#[proc_macro_derive(Service)]
pub fn service_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_service_macro(&ast)
}

fn impl_service_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!("Service macro can only be used with structs with named fields"),
        },
        _ => unimplemented!("Service macro can only be used with structs"),
    };

    let gen_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        // Check if the type is an Arc and extract the inner type if it is
        let inner_type = if let Type::Path(type_path) = field_type {
            if type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == "Arc" {
                if let PathArguments::AngleBracketed(ref args) =
                    type_path.path.segments[0].arguments
                {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        Some(inner_ty)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        match inner_type {
            Some(it) => {
                if let Type::TraitObject(_) = it {
                    quote! {
                        let #field_name = shive::service::get_trait_instance::<#inner_type>(service_provider)
                            .expect("Cannot get trait type from service manager");
                    }
                }
                else {
                    quote! {
                        let #field_name = shive::service::get_instance::<#inner_type>(service_provider)
                            .expect("Cannot get type from service manager");
                    }
                }
            }
            None => {
                unimplemented!("Struct property type must be inside an Arc")
            }
        }
    });

    let gen_field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name,
        }
    });

    let gen_service = quote! {
        impl Service for #name {
            fn init(service_provider: &dyn shive::service::ServiceResolver) -> Arc<dyn shive::service::Service>
            where
                Self: Sized,
            {
                #(#gen_fields)*

                Arc::new(Self { #(#gen_field_names)* })
            }

            fn as_any(self: Arc<Self>) -> Arc<dyn std::any::Any + Send + Sync> {
                self
            }
        }
    };

    gen_service.into()
}
