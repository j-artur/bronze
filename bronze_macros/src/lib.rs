use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{
    parse, parse::Parser, parse_macro_input, Data, DataStruct, DeriveInput, Fields, ItemStruct,
    Type,
};

#[proc_macro_attribute]
pub fn position(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                   position: bronze::entity::Position
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[proc_macro_attribute]
pub fn size(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    size: bronze::entity::Size
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[proc_macro_attribute]
pub fn rotation(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    rotation: bronze::entity::Rotation
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[proc_macro_attribute]
pub fn scale(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    scale: bronze::entity::Scale
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[proc_macro_derive(Entity)]
pub fn entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics.params;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let mut gen = quote! {};

    let has_position = fields
        .iter()
        .find(|f| f.ident.as_ref().unwrap() == "position")
        .map(|f| match f.ty {
            Type::Path(ref path) => path.path.segments.last().unwrap().ident == "Position",
            _ => false,
        })
        .is_some();

    gen.append_all(if has_position {
        quote! {
            impl<#generics> bronze::entity::EntityPosition for #name<#generics> {
                #[inline]
                fn position(&self) -> bronze::entity::Position {
                    self.position
                }
                #[inline]
                fn set_position(&mut self, x: f32, y: f32) {
                    self.position = bronze::entity::Position::new(x, y);
                }
                #[inline]
                fn move_by(&mut self, x: f32, y: f32) {
                    self.position += bronze::entity::Position::new(x, y);
                }
            }
        }
    } else {
        quote! {
            impl<#generics> bronze::entity::EntityPosition for #name<#generics> {
                #[inline]
                fn position(&self) -> bronze::entity::Position {
                    bronze::entity::Position::new(0.0, 0.0)
                }
                #[inline]
                fn set_position(&mut self,  _x: f32, y: f32) {}
                #[inline]
                fn move_by(&mut self, _x: f32, _y: f32) {}
            }
        }
    });

    let has_size = fields
        .iter()
        .find(|f| f.ident.as_ref().unwrap() == "size")
        .map(|f| match f.ty {
            Type::Path(ref path) => path.path.segments.last().unwrap().ident == "Size",
            _ => false,
        })
        .is_some();

    gen.append_all(if has_size {
        quote! {
            impl<#generics> bronze::entity::EntitySize for #name<#generics> {
                #[inline]
                fn size(&self) -> bronze::entity::Size {
                    self.size
                }
                #[inline]
                fn set_size(&mut self, x: f32, y: f32) {
                    self.size = bronze::entity::Size::new(x, y);
                }
            }
        }
    } else {
        quote! {
            impl<#generics> bronze::entity::EntitySize for #name<#generics> {
                #[inline]
                fn size(&self) -> bronze::entity::Size {
                    bronze::entity::Size::new(0.0, 0.0)
                }
                #[inline]
                fn set_size(&mut self, _x: f32, _y: f32) {}
            }
        }
    });

    let has_rotation = fields
        .iter()
        .find(|f| f.ident.as_ref().unwrap() == "rotation")
        .map(|f| match &f.ty {
            Type::Path(ref path) => path.path.segments.last().unwrap().ident == "Rotation",
            _ => false,
        })
        .is_some();

    gen.append_all(if has_rotation {
        quote! {
            impl<#generics> bronze::entity::EntityRotation for #name<#generics> {
                #[inline]
                fn rotation(&self) -> bronze::entity::Rotation {
                    self.rotation
                }
                #[inline]
                fn set_rotation(&mut self, rotation: bronze::entity::Rotation) {
                    self.rotation = rotation;
                }
                #[inline]
                fn rotate_by(&mut self, offset: bronze::entity::Rotation) {
                    self.rotation += offset;
                }
            }
        }
    } else {
        quote! {
            impl<#generics> bronze::entity::EntityRotation for #name<#generics> {
                #[inline]
                fn rotation(&self) -> bronze::entity::Rotation {
                    0.0
                }
                #[inline]
                fn set_rotation(&mut self, _rotation: bronze::entity::Rotation) {}
                #[inline]
                fn rotate_by(&mut self, _offset: bronze::entity::Rotation) {}
            }
        }
    });

    let has_scale = fields
        .iter()
        .find(|f| f.ident.as_ref().unwrap() == "scale")
        .map(|f| match &f.ty {
            Type::Path(ref path) => path.path.segments.last().unwrap().ident == "Scale",
            _ => false,
        })
        .is_some();

    gen.append_all(if has_scale {
        quote! {
            impl<#generics> bronze::entity::EntityScale for #name<#generics> {
                #[inline]
                fn scale(&self) -> bronze::entity::Scale {
                    self.scale
                }
                #[inline]
                fn set_scale(&mut self, x: f32, y: f32) {
                    self.scale = bronze::entity::Scale::new(x, y);
                }
                #[inline]
                fn scale_by(&mut self, x: f32, y: f32) {
                    self.scale.x *= x;
                    self.scale.y *= y;
                }
            }
        }
    } else {
        quote! {
            impl<#generics> bronze::entity::EntityScale for #name<#generics> {
                #[inline]
                fn scale(&self) -> bronze::entity::Scale {
                    bronze::entity::Scale::new(1.0, 1.0)
                }
                #[inline]
                fn set_scale(&mut self, _x: f32, _y: f32) {}
                #[inline]
                fn scale_by(&mut self, _x: f32, _y: f32) {}
            }
        }
    });

    gen.into()
}
