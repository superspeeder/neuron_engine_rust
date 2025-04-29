use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::LitStr;


///
/// Use this attribute on a function to designate it the entry point for the plugin (responsible for constructing the `Plugin` object).
///
/// You can only use this on **one** function within a single plugin shared library.
///
#[proc_macro_attribute]
pub fn plugin_entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_ident = item.sig.ident.clone();

    let apicrate = crate_name("neuron_script_api").expect("neuron_script_api is present in `Cargo.toml`");

    let crate_ident = match &apicrate {
        FoundCrate::Itself => return quote! { compile_error!("Can't create a plugin entry in script api implementation"); }.into(),
        FoundCrate::Name(name) => {
            Ident::new(name, Span::call_site())
        }
    };

    let pcc_ident = quote! { #crate_ident::plugin::PluginCreationContext };
    let pcont_ident = quote! { #crate_ident::plugin::PluginContainer };

    let implementor_name = std::env::var("CARGO_PKG_NAME").unwrap();

    let fail_msg_m = format!("Failed to create plugin `{}`", implementor_name);
    let null_pcc_m = format!("Plugin entry point for `{}` failed: null creation context passed by runtime.", implementor_name);

    let null_pcc_msg = LitStr::new(&null_pcc_m, Span::call_site());
    let fail_msg = LitStr::new(&fail_msg_m, Span::call_site());

    quote! {
        #item

        // generated entry point
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn _plugin_entry(plugin_creation_context: *mut #pcc_ident) -> *mut #pcont_ident {
            use std::boxed::Box;
            use std::ptr::NonNull;
            use std::option::Option;
            let Some(pcc) = NonNull::<#pcc_ident>::new(plugin_creation_context) else { panic!(#null_pcc_msg) };
            pcc.as_ref().generic_setup();
            Box::leak(Box::new(#pcont_ident(Box::new(#fn_ident(pcc).expect(#fail_msg)))))
        }
    }.into()
}