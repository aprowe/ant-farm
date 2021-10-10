use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields,
};

struct BreederSpec {
    name: syn::Ident,
    ty: syn::Type,
    weight: f32,
}

fn remove_attrs(data: &mut Data){
    match *data {
        Data::Struct(ref mut data) => match data.fields {
            Fields::Named(ref mut fields) => fields
                .named
                .iter_mut()
                .map(|f| {
                    f.attrs.clear();
                }).collect(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn get_breeders(data: &Data) -> Vec<BreederSpec> {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .filter(|f| f.attrs.first().unwrap().path.is_ident("breeder"))
                .map(|f| {
                    let tokens = &f.attrs.first().unwrap().tokens;
                    let fstring = tokens.to_string();
                    let weight: f32 = fstring[1..fstring.len() - 1].parse().unwrap();

                    BreederSpec {
                        ty: f.ty.clone(),
                        name: f.ident.clone().unwrap(),
                        weight,
                    }
                })
                .collect(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn create_genome(ident: &syn::Ident, breeders: &Vec<BreederSpec>) -> TokenStream {
    let genome_name = format_ident!("{}Genome", ident.to_string());
    let fields = breeders.iter().map(|b| {
        let BreederSpec {name, ty, ..} = b;

        quote! {
            pub #name: <#ty as Breeder>::Genome,
        }
    });
    let code = quote! {
        #[derive(Clone, Debug)]
        pub struct #genome_name {
            #(#fields)*
        }
    };
    code.into()
}

fn create_impl(ident: &syn::Ident, breeders: &Vec<BreederSpec>) -> TokenStream {
    let mutate = breeders.iter().map(|b| {
        let BreederSpec {name, weight, .. } = b;

        quote! {
            #name: if rand::random::<f32>() < #weight {
                self.#name.mutate(&g.#name)
            } else {
                g.#name.clone()
            },
        }
    });

    let breed = breeders.iter().map(|b| {
        let BreederSpec {name, weight, .. } = b;

        quote! {
            #name: if rand::random::<f32>() < #weight {
                self.#name.breed(&g1.#name, &g2.#name)
            } else if rand::random::<f32>() < 0.5 {
                g1.#name.clone()
            } else {
                g2.#name.clone()
            },
        }
    });

    let random = breeders.iter().map(|b| {
        let BreederSpec {name, ..} = b;
        quote! {
            #name: self.#name.random(),
        }
    });

    let is_same = breeders.iter().map(|b| {
        let BreederSpec {name, ..} = b;
        quote! {
            self.#name.is_same(&g1.#name, &g2.#name)
        }
    });

    let genome = format_ident!("{}Genome", ident.to_string());
    let code = quote! {
        impl Breeder for #ident {
            type Genome = #genome;

            fn mutate(&self, g: &Self::Genome) -> Self::Genome {
                Self::Genome {
                    #(#mutate)*
                }
            }

            fn breed(&self, g1: &Self::Genome, g2: &Self::Genome) -> Self::Genome {
                Self::Genome {
                    #(#breed)*
                }
            }

            fn random(&self) -> Self::Genome {
                Self::Genome {
                    #(#random)*
                }
            }

            fn is_same(&self, g1: &Self::Genome, g2: &Self::Genome) -> bool {
                true #(&& #is_same)*
            }
        }

        // gene.fn style calls
        impl #genome {
            fn mutate(&self, breeder: &#ident) -> Self {
                breeder.mutate(&self)
            }
            fn breed(&self, breeder: &#ident, other: &Self) -> Self {
                breeder.breed(&self, other)
            }
            fn random(breeder: &#ident) -> Self {
                breeder.random()
            }
            fn is_same(&self, breeder: &#ident, other: &Self) -> bool {
                breeder.is_same(self, other)
            }
        }
    };
    code.into()
}

#[proc_macro_attribute]
pub fn derive_breeder(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    let breeders = get_breeders(&input.data);
    let breeder_impl = create_impl(&input.ident, &breeders);
    let genome = create_genome(&input.ident, &breeders);

    remove_attrs(&mut input.data);

    let code = quote! {
        #input
        #genome
        #breeder_impl
    };
    // println!("{}", code.to_string());
    code.into()
}
