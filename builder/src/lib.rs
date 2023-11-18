use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use proc_macro2::{Ident, Span};
use syn::parse_macro_input;
use quote::ToTokens;
use std::error::Error;



// impl_into_system


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // eprintln!("original_input: {:#?}", input); 
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    // eprintln!("derive_input: {:#?}", derive_input); 
    // Command
    let struct_name = derive_input.ident;
    // CommandBuilder
    let temp_ident = Ident::new(&format!("{}Builder", struct_name), Span::call_site());
    // eprintln!();
    // x.to_token_stream().into()
    if let syn::Data::Struct(data) = derive_input.data {
        // eprintln!("CRYPTIC: {:#?}", data); 
        // i can cycle through all the fields in the struct and determine whether a field is an
        // option or not. based on this i can construct the new struct. this is the way in which i
        // should aim to solve this problem
        // eprintln!("xd: {:#?}", fields); 
        if let syn::Fields::Named(fields) = data.fields{
            // punctuated type
            for x in fields.named.iter(){
                eprintln!("xd: {:#?}", x); 

                // grab the name
                if let Some(ident) = &x.ident { //why do i need a borrow here but not when im
                                                //getting the fields. maybe enum have something to
                                                //do with it. I woudl assume string would have a
                                                //copy method
                                                // maybe because im calling a method on string????
                                                //
                                                // The value T of Option<T> gets moved when
                                                // matching. But I'm not using them down below just
                                                // yet that's way I can get away with this...
                                                // clearly
                    let name = ident.to_string();
                    eprintln!("xd_name: {:#?}", name); 
                }
            }
            // if let syn::Fields::Named(fields) = data.fields{
            //     
            // } 
        } 
    }


    let token = quote!(
        #[derive(Debug)]
        pub struct #temp_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #temp_ident {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            
            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

             pub fn build(&mut self) -> Result<#struct_name, ()> {
                 let executable = self.executable.as_ref().unwrap();
                 let args = self.args.as_ref().unwrap();
                 let env = self.env.as_ref().unwrap();
                 let current_dir = self.current_dir.as_ref().unwrap();
            
                 Ok(#struct_name{
                     executable: executable.to_string(),
                     args: args.to_vec(),
                     env: env.to_vec(),
                     current_dir: current_dir.to_string(),
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
    // eprintln!("token: {:#?}", token); 
    token.into()
}

// TODO: take this outta here!
// TODO: take a look at: ```let executable = self.executable.as_ref().unwrap(); ```
// TODO: use quote! outside a fucking macro
//
// TODO: see pub fn derive_as_bind_group(input: TokenStream) -> TokenStream
// - there is a trait AsBindGroup with 7 functions. The macro implements 2 functions
//   https://docs.rs/bevy/latest/bevy/render/render_resource/trait.AsBindGroup.html
// - crates/bevy_render/macros/src/as_bind_group.rs
//

