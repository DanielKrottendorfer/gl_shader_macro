
extern crate cgmath;

use quote::{quote};
use proc_macro2::{TokenTree::*};
#[allow(dead_code)]
mod templates;
use syn::parse_quote;
use templates::*;

#[proc_macro]
pub fn shader_program(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let mut input = proc_macro2::TokenStream::from(_item).into_iter();

    let struct_ident = if let Ident(i) = input.next().unwrap() {i} else {panic!("no shader name")};
    
    let uniforms = if let Group(g) = input.next().unwrap() {g.stream()} else {panic!("no uniforms")};

    let mut field_idents = Vec::new();
    let mut uniform_setup = proc_macro2::TokenStream::new();
    let mut set_uniform_functions = proc_macro2::TokenStream::new();

    for s in uniforms.to_string().split(';') {
        let mut comp = s.trim().split(&[' ','\n']);
        if let Some(c) = comp.next() {
            let c = c.trim();
            if c.starts_with("uniform") {
                let uniform_type = comp.next().unwrap();
                let uniform_name = comp.next().unwrap();
                
                let field_ident = quote::format_ident!("{}",uniform_name);
                uniform_setup.extend(setup_uniform(&field_ident,uniform_name));
                
                let set_uniform_function_name = quote::format_ident!("set_{}",field_ident);
                let set_uniform_function = match uniform_type.trim() {
                    "float" => f1_setter     (&set_uniform_function_name, &field_ident, &field_ident),
                    "int"   => i1_setter     (&set_uniform_function_name, &field_ident, &field_ident),
                    "i1v"   => i1v_setter    (&set_uniform_function_name, &field_ident, &field_ident),
                    "vec2"  => vec2_setter   (&set_uniform_function_name, &field_ident, &field_ident),
                    "vec3"  => vec3_setter   (&set_uniform_function_name, &field_ident, &field_ident),
                    "ivec3"  => ivec3_setter  (&set_uniform_function_name, &field_ident, &field_ident),
                    "vec4"  => vec4_setter   (&set_uniform_function_name, &field_ident, &field_ident),
                    "mat4"  => mat4_setter   (&set_uniform_function_name, &field_ident, &field_ident),
                    _ => {
                        panic!("uniform type not recognized => {}",uniform_type);
                    }
                };

                field_idents.push(field_ident);
                set_uniform_functions.extend(set_uniform_function);
            }
        }
    }

    let output = quote! {
        #[derive(Debug,Default,Copy,Clone)]
        pub struct #struct_ident{
            pub program_id :u32,
            #(pub #field_idents :i32,)*
        }
        impl #struct_ident{
            pub fn new() -> Self {
                #struct_ident {
                    program_id:0,
                    #( #field_idents :-1,)*
                }
            }
            pub fn use_program(&self){
                unsafe{
                    gl::UseProgram(self.program_id);
                }
            }
            pub fn setup(&mut self,program: &u32){
                self.program_id = program.clone();
                #uniform_setup
            }
            pub fn cleanup(&self){
                unsafe {
                    gl::DeleteProgram(self.program_id);
                }
            }
            #set_uniform_functions
        }
    };

    #[cfg(feature = "print_output")]
    println!("{}",output.to_string());

    proc_macro::TokenStream::from(output)
}
