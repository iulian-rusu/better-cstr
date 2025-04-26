use proc_macro::TokenStream;
use quote::quote;
use std::ffi::CString;
use syn::{parse_macro_input, Error, Lit, LitCStr};

/// Transforms a string/byte string literal into a C-string pointer (`*const c_char`).
/// 
/// Generates a compile-time error if the provided literal cannot be converted to a C-string (e.g. if it contains null bytes).
/// # Examples
/// 
/// ```
/// use better_cstr::c;
/// let ptr = c!("test");
/// ```
/// 
/// The above code is equivalent to:
/// 
/// ```
/// use std::ffi::c_char;
/// let ptr: *const c_char = c"test".as_ptr();
/// ```
#[proc_macro]
pub fn c(tokens: TokenStream) -> TokenStream {
    let lit: Lit = parse_macro_input!(tokens);
    let span = lit.span();
    let cstring = match lit {
        Lit::Str(lit_str) => {
            CString::new(lit_str.value()).expect("String literal must be a valid C string")
        }
        Lit::ByteStr(lit_byte_str) => {
            let mut bytes = lit_byte_str.value();
            bytes.push(0u8);
            CString::from_vec_with_nul(bytes).expect("Byte string literal must be a valid C string")
        }
        _ => {
            return Error::new_spanned(lit, "Unsupported literal kind")
                .to_compile_error()
                .into()
        }
    };
    let lit_cstr = LitCStr::new(cstring.as_c_str(), span);
    quote! {
        #lit_cstr.as_ptr()
    }
    .into()
}
