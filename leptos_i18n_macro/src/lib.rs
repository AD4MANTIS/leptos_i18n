#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![cfg_attr(feature = "nightly", feature(proc_macro_diagnostic))]
//! # About Leptos i18n macro
//!
//! This crate expose the utility macro for `leptos_i18n`
//!
//! This crate must be used with `leptos_i18n` and should'nt be used outside of it.

pub(crate) mod load_locales;
pub(crate) mod t_macro;

// for deserializing the files custom deserialization is done,
// this is to use `serde::de::DeserializeSeed` to pass information on what locale or key we are currently at
// and give better information on what went wrong when an error is emitted.

/// Look for the configuration in the cargo manifest `Cargo.toml` at the root of the project and load the given locales.
///
/// It creates multiple types allowing to easily incorporate translations in you application such as:
///
/// - `Locale`: an enum representing the available locales of the application.
/// - `I18nKeys`: a struct representing the translation keys.
#[proc_macro]
pub fn load_locales(_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match load_locales::load_locales() {
        Ok(ts) => ts.into(),
        Err(err) => err.into(),
    }
}

/// Utility macro to easily put translation in your application.
///
/// Usage:
///
/// ```rust, ignore
/// use crate::i18n::*;
///
/// let i18n = use_i18n();
///
/// view! {
///     <p>{t!(i18n, $key)}</p>
///     <p>{t!(i18n, $key, $variable = $value, <$component> = |child| ... )}</p>
/// }
///```
///
/// # Notes
///
/// If your variable/component value is the same as the key, you remove the assignement, such that this:
///
/// ```rust, ignore
/// t!(i18n, $key, variable = variable, <component> = component, $other_key = $other_value, ..)
/// ```
///
/// can be shortened to:
///
/// ```rust, ignore
/// t!(i18n, $key, variable, <component>, $other_key = $other_value, ..)
/// ```
#[proc_macro]
pub fn t(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    t_macro::t_macro(tokens, false)
}

/// Just like the `t!` macro but instead of taking `I18nContext` as the first argument it takes the desired locale.
///
/// Usage:
///
/// ```rust, ignore
/// use crate::i18n::Locale;
/// use leptos_i18n::td;
///
/// view! {
///     <p>{td!(Locale::en, $key)}</p>
///     <p>{td!(Locale::fr, $key, $variable = $value, <$component> = |child| ... )}</p>
/// }
///```
///
/// This let you use a specific locale regardless of the current one.
#[proc_macro]
pub fn td(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    t_macro::t_macro(tokens, true)
}
