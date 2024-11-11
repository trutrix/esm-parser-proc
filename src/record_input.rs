use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, FieldsNamed, Ident, LitByteStr, LitInt, LitStr, Token};



pub struct RecordInput {
    pub name: LitStr
}


impl Parse for RecordInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
        let _name_label: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?;
        input.parse::<Token![;]>()?;

        let _iden_label: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let iden: LitByteStr = input.parse()?;
        input.parse::<Token![;]>()?;

        let _flags_label: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let inner;
        braced!(inner in input);

        let flags = inner.parse_terminated(FlagDefinition::parse, Token![;])?;

        input.parse::<Token![;]>()?;


        let _fields_iden: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let inner;
        braced!(inner in input);


        input.parse::<Token![;]>()?;

        Ok(RecordInput { name })
    }
}


impl ToTokens for RecordInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(format!("{}Field", self.name.value().as_str()).as_str(), self.name.span());

        let mut out=  quote! {
            pub struct #name {

            }
        };

        tokens.extend(out);
    }
}


pub struct FlagDefinition {
    pub id: Ident,
    pub pos: LitInt
}

impl Parse for FlagDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let id: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let pos: LitInt = input.parse()?;
        Ok(FlagDefinition { id, pos })
    }
}