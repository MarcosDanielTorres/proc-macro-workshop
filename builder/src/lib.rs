use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::parse_macro_input;

fn type_from_inside_option(ty: &syn::Type) -> Option<&syn::Type> {
    let path = if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_some() {
            return None;
        }
        &type_path.path
    } else {
        return None;
    };
    let segment = path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let generic_params =
        if let syn::PathArguments::AngleBracketed(generic_params) = &segment.arguments {
            generic_params
        } else {
            return None;
        };
    if let syn::GenericArgument::Type(ty) = generic_params.args.first()? {
        Some(ty)
    } else {
        None
    }
}

fn method_logic(name: &Option<Ident>, ty: &syn::Type) -> TokenStream {
    quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
    }
}
// for attr in &field.attrs {
//             let Some(attr_ident) = attr.path().get_ident() else {
//                 continue;
//             };
//             // get_ident() == builder
//             // builder(each =  "arg")
//             #[uniform(0)]
//             let binding_type = if attr_ident == UNIFORM_ATTRIBUTE_NAME {
//                 BindingType::Uniform
//             } else if attr_ident == TEXTURE_ATTRIBUTE_NAME {
//                 BindingType::Texture
//             } else if attr_ident == SAMPLER_ATTRIBUTE_NAME {
//                 BindingType::Sampler
//             } else if attr_ident == STORAGE_ATTRIBUTE_NAME {
//                 BindingType::Storage

fn method_impl_sum(input: &syn::DeriveInput, data: &syn::Data) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut impl_method_vec = Vec::new();
    let mut build_method_vec = Vec::new();
    let mut result_ok_vec = Vec::new(); // if it has an atribute called `each` i will create a method that takes only one variable
                                        // if it hasn't i will use the normal flow of taking a vector

    let _ = match data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => {
                for field in fields.named.iter() {

                    let name = &field.ident;
                    let ty = &field.ty;

                     for a in input.attrs.iter(){
                         if a.path().is_ident("command"){
                             eprintln!("ATTR: command");
                         }
                    
                     }

                    // for attr in field.attrs.iter(){
                    //     if attr.path().is_ident("builder"){
                    //         eprintln!("ATTR: builder");
                    //
                    //     }
                    // }

                    if !field.attrs.is_empty() {
                        // I know there is not going to be more than one attr for know
                        match &field.attrs.last().unwrap().meta {
                            syn::Meta::Path(path) => {
                                unimplemented!();
                            }
                            syn::Meta::List(metalist) => {
                                let x = &metalist;
                                // eprintln!("META LIST: {:#?}", x.tokens.to_string());
                                // dbg!(x);
                            }
                            syn::Meta::NameValue(metanamevalue) => {
                                unimplemented!();
                            }
                        };
                    }

                    let type_inside_option = type_from_inside_option(ty); // String -> None

                    let (is_inside_option, ty) = match type_inside_option {
                        Some(val) => (true, val),
                        None => (false, ty),
                    };

                    let impl_methods = method_logic(name, ty);

                    impl_method_vec.push(impl_methods);

                    let build_methods;
                    let result_ok_sentences;

                    if is_inside_option {
                        build_methods = quote! {
                            let #name = self.#name.clone();
                        };

                        result_ok_sentences = quote! {
                            #name: #name.clone(),
                        };
                    } else {
                        build_methods = quote! {
                            let Some(#name) = self.#name.take() else{
                                panic!("JAJA")
                            };
                        };

                        result_ok_sentences = quote! {
                            #name: #name,
                        };
                    }

                    build_method_vec.push(build_methods);

                    result_ok_vec.push(result_ok_sentences);
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    return (impl_method_vec, build_method_vec, result_ok_vec);
}

#[proc_macro_derive(Builder, attributes(builder, command))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    // Command
    let struct_name = &input.ident;
    // CommandBuilder
    let temp_ident = Ident::new(&format!("{}Builder", struct_name), Span::call_site());

    let (impl_method, build_method, result_ok_vec) = method_impl_sum(&input, &input.data);

    let token = quote!(
        #[derive(Debug)]
        pub struct #temp_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #temp_ident {
            #(#impl_method)*

            pub fn build(&mut self) -> Result<#struct_name, ()> {
                #(#build_method)*

                Ok(#struct_name {
                    #(#result_ok_vec)*
                })
            }
        }

        impl #struct_name {
            pub fn builder() -> #temp_ident{
                 #temp_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }

        }
    );
    proc_macro::TokenStream::from(token)
}

// TODO: take this outta here!
// TODO: take a look at: ```let executable = self.executable.as_ref().unwrap(); ```
// TODO: use quote! outside a fucking macro
//
// TODO: see pub fn derive_as_bind_group(input: TokenStream) -> TokenStream
// - there is a trait AsBindGroup with 7 functions. The macro implements 2 functions
//   https://docs.rs/bevy/latest/bevy/render/render_resource/trait.AsBindGroup.html
//   crates/bevy_render/macros/src/as_bind_group.rs
//
// TODO: check get_visibility_flag_value that uses another macro for the visibility of crates
// get_storage_binding_attr is one of the callers
