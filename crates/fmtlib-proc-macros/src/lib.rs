use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned, Expr, Lit, Token,
};

enum ArgMacroInput {
    Pos(Expr),
    Named(Expr, Expr),
}

impl ArgMacroInput {
    /// Parses either an expression matching V, or an expression matching K: V
    /// where V is an expression that evaluates to a value and K is an identifier
    /// or an expression that evaluates to an identifier.
    fn parse_alternate(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Has ':' not followed by ':'
        let res = if input.peek2(Token![:]) && !input.peek3(Token![:]) {
            // Named
            let key: Expr = input.parse()?;

            // Ignore ':' token
            let _: Token![:] = input.parse()?;
            let value: Expr = input.parse()?;

            Self::Named(key, value)
        } else {
            let value: Expr = input.parse()?;
            Self::Pos(value)
        };

        Ok(res)
    }
}

impl ToTokens for ArgMacroInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = quote! {Arg};
        match self {
            ArgMacroInput::Pos(value) => {
                let arg = expand_value(value);
                tokens.extend(quote! {#ty::positional(#arg)});
            }
            ArgMacroInput::Named(key, value) => {
                let k = expand_key(key);
                let v = expand_value(value);
                tokens.extend(quote! {#ty::named(#k, #v)})
            }
        }
    }
}

fn expand_key(key: &Expr) -> proc_macro2::TokenStream {
    if let Expr::Path(path) = key {
        if let Some(ident) = path.path.get_ident() {
            let val = ident.to_string();
            return quote_spanned! { ident.span() =>
                #val
            };
        }
    } else if let Expr::Lit(expr) = key {
        // Named argument keys must be C strings
        let expr = &expr.lit;
        if let Lit::Int(v) = expr {
            let val = v.base10_digits();
            return quote_spanned! { v.span() =>
                #val
            };
        } else if let Lit::Float(v) = expr {
            return syn::Error::new(v.span(), "bool literals are not supported as keys")
                .to_compile_error();
        } else if let Lit::Bool(v) = expr {
            return syn::Error::new(v.span(), "bool literals are not supported as keys")
                .to_compile_error();
        }
    }

    expand_value(key)
}

fn expand_value(value: &Expr) -> proc_macro2::TokenStream {
    match value {
        Expr::Array(v) => {
            syn::Error::new(v.span(), "array expressions are not supported").to_compile_error()
        }
        Expr::Assign(v) => {
            syn::Error::new(v.span(), "assignment expressions are not supported").to_compile_error()
        }
        Expr::Async(v) => {
            syn::Error::new(v.span(), "async expressions are not supported").to_compile_error()
        }
        Expr::Await(v) => quote_spanned! {v.span() => #v},
        Expr::Binary(v) => quote_spanned! {v.span() => #v},
        Expr::Block(v) => quote_spanned! {v.span() => #v},
        Expr::Break(v) => {
            syn::Error::new(v.span(), "break expressions are not supported").to_compile_error()
        }
        Expr::Call(v) => quote_spanned! {v.span() => #v},
        Expr::Cast(v) => quote_spanned! {v.span() => #v},
        Expr::Closure(v) => {
            syn::Error::new(v.span(), "closure expressions are not supported").to_compile_error()
        }
        Expr::Const(v) => quote_spanned! {v.span() => #v},
        Expr::Continue(v) => {
            syn::Error::new(v.span(), "continue expressions are not supported").to_compile_error()
        }
        Expr::Field(v) => quote_spanned! {v.span() => #v},
        Expr::ForLoop(v) => {
            syn::Error::new(v.span(), "for loop expressions are not supported").to_compile_error()
        }
        Expr::Group(v) => quote_spanned! {v.span() => #v},
        Expr::If(v) => quote_spanned! {v.span() => #v},
        Expr::Index(v) => quote_spanned! {v.span() => #v},
        Expr::Infer(v) => {
            syn::Error::new(v.span(), "'_' expressions are not supported").to_compile_error()
        }
        Expr::Let(v) => {
            syn::Error::new(v.span(), "let expressions are not supported").to_compile_error()
        }
        Expr::Lit(v) => expand_literal(&v.lit),
        Expr::Loop(v) => {
            syn::Error::new(v.span(), "loop expressions are not supported").to_compile_error()
        }
        Expr::Macro(v) => quote_spanned! {v.span() => #v},
        Expr::Match(v) => quote_spanned! {v.span() => #v},
        Expr::MethodCall(v) => quote_spanned! {v.span() => #v},
        Expr::Paren(v) => quote_spanned! {v.span() => #v},
        Expr::Path(v) => quote_spanned! {v.span() => #v},
        Expr::Range(v) => {
            syn::Error::new(v.span(), "range expressions are not supported").to_compile_error()
        }
        Expr::Reference(v) => quote_spanned! {v.span() => #v},
        Expr::Repeat(v) => syn::Error::new(v.span(), "array literal expressions are not supported")
            .to_compile_error(),
        Expr::Return(v) => {
            syn::Error::new(v.span(), "return expressions are not supported").to_compile_error()
        }
        Expr::Struct(v) => {
            syn::Error::new(v.span(), "struct expressions are not supported").to_compile_error()
        }
        Expr::Try(v) => quote_spanned! {v.span() => #v},
        Expr::TryBlock(v) => {
            syn::Error::new(v.span(), "try block expressions are not supported").to_compile_error()
        }
        Expr::Tuple(v) => quote_spanned! {v.span() => #v.into()},
        Expr::Unary(v) => quote_spanned! {v.span() => #v.into()},
        Expr::Unsafe(v) => quote_spanned! {v.span() => #v},
        Expr::Verbatim(v) => {
            syn::Error::new(v.span(), "array expressions are not supported").to_compile_error()
        }
        Expr::While(v) => syn::Error::new(v.span(), "while loops expressions are not supported")
            .to_compile_error(),
        Expr::Yield(v) => {
            syn::Error::new(v.span(), "yield expressions are not supported").to_compile_error()
        }

        _ => syn::Error::new(value.span(), "expression is not supported").to_compile_error(),
    }
}

fn expand_literal(literal: &Lit) -> proc_macro2::TokenStream {
    match literal {
        Lit::Str(v) => quote_spanned! {v.span() => #v},
        Lit::ByteStr(v) => {
            syn::Error::new(v.span(), "byte string literals are not supported").to_compile_error()
        }
        Lit::Byte(v) => {
            syn::Error::new(v.span(), "byte literals are not supported").to_compile_error()
        }
        Lit::Char(v) => {
            // Rust chars aren't compatoble with C chars. convert to a string.
            let val = v.value().to_string();
            quote_spanned! {v.span() => #val}
        }
        Lit::Int(v) => quote_spanned! {v.span() => #v},
        Lit::Float(v) => quote_spanned! {v.span() => #v},
        Lit::Bool(v) => quote_spanned! {v.span() => #v},
        Lit::Verbatim(v) => quote_spanned! {v.span() => #v},
        _ => unreachable!("unknown literal kind"),
    }
}

struct ArgsMacroInput {
    pos: Punctuated<ArgMacroInput, Token![,]>,
    named: Punctuated<ArgMacroInput, Token![,]>,
}

impl Parse for ArgsMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut pos = Punctuated::new();
        let mut named = Punctuated::new();
        loop {
            if input.is_empty() {
                break;
            }
            let value = ArgMacroInput::parse_alternate(input)?;
            let is_pos = match value {
                ArgMacroInput::Pos(_) => {
                    pos.push_value(value);
                    true
                }
                ArgMacroInput::Named(_, _) => {
                    named.push_value(value);
                    false
                }
            };
            if input.is_empty() {
                break;
            }
            let punct = input.parse()?;
            if is_pos {
                pos.push_punct(punct);
            } else {
                named.push_punct(punct);
            }
        }
        Ok(Self { pos, named })
    }
}

/// Used by the format! macro to create an array of args.
#[proc_macro]
pub fn args(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ArgsMacroInput);
    let mut out_stream = proc_macro2::TokenStream::new();
    let count = parsed.pos.len() + parsed.named.len();
    out_stream.append_terminated(
        parsed.pos,
        proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
    );
    // named args should be last.
    out_stream.append_separated(
        parsed.named,
        proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
    );

    if count > 1024 {
        quote! {
            vec![
                #out_stream
            ]
        }
    } else if count > 0 {
        quote! {
            [
                #out_stream
            ]
        }
    } else {
        quote! {
            [] as [Arg; 0]
        }
    }
    .into()
}
