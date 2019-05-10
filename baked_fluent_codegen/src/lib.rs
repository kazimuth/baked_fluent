#![recursion_limit = "128"]

//! Internationalization codegen.

extern crate proc_macro;

use quote::quote;
use std::env;
use std::fs::{read_to_string, DirEntry};
use std::path::{Path, PathBuf};

mod error;
mod input;

macro_rules! err {
    ($span:expr, $message:expr) => {
        return syn::Error::new($span, $message).to_compile_error().into();
    };
}

/// The impl_localize macro.
#[proc_macro]
pub fn impl_localize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse input.
    let ast = syn::parse_macro_input!(item as input::ImplLocalize);

    // load all source files.
    let mut root = PathBuf::from(
        &env::var("CARGO_MANIFEST_DIR").expect("baked_fluent doesn't work without cargo"),
    );
    root.push(&ast.path.value());
    let sources = match collect_sources(&root) {
        Some(sources) => sources,
        None => err!(
            ast.path.span(),
            "baked_fluent: .ftl source files have errors"
        ),
    };

    let locales = sources
        .iter()
        .map(|(locale, _, _)| locale)
        .collect::<Vec<_>>();

    if let None = locales
        .iter()
        .find(|locale| **locale == &ast.default_locale.value())
    {
        err!(
            ast.default_locale.span(),
            format!(
                "baked_fluent: no translations for default locale {:?} \
                 (have: {:?})",
                ast.default_locale.value(),
                locales
            )
        );
    }

    // setup for invocation of quote
    let name = ast.name;
    let default_locale = ast.default_locale;
    let includes = sources.iter().flat_map(|s| &s.1);
    let sources = sources.iter().map(|(locale, _, sources)| {
        quote! {
            (#locale, &[#(#sources)*])
        }
    });

    // generated code
    (quote! {
        /// Internationalization support. Automatically generated from files in the `i18n` folder.
        /// For usage, see the docs of the baked_fluent::Localize trait.
        pub struct #name(Box<[&'static str]>);

        impl ::baked_fluent::Localize for #name {
            #[inline(never)]
            fn new(locale: &[&str], accept_language: Option<&str>) -> Self {
                #name(
                    __i18n_hidden::STATIC_PARSER
                        .create_locale_chain(locale, accept_language)
                        .into_boxed_slice(),
                )
            }

            #[inline]
            fn localize_into<W: std::fmt::Write>(
                &self,
                writer: &mut W,
                message: &'static str,
                args: &[(&str, &::baked_fluent::runtime::I18nValue)],
            ) -> ::baked_fluent::Result<()> {
                __i18n_hidden::STATIC_PARSER.localize_into(writer, &self.0, message, args)
            }

            fn has_message(&self, message: &'static str) -> bool {
                __i18n_hidden::STATIC_PARSER.has_message(&self.0, message)
            }

            fn default_locale() -> &'static str {
                #default_locale
            }
        }

        #[doc(hidden)]
        mod __i18n_hidden {
            use baked_fluent::runtime::{lazy_static, I18nValue, Resources, Sources, StaticParser};

            /// All sources compiled into the executable.
            pub const SOURCES: Sources = &[
                 #(#sources),*
            ];

            /// The parsed sources.
            lazy_static! {
                static ref RESOURCES: Resources = Resources::new(SOURCES);
                pub static ref STATIC_PARSER: StaticParser<'static> =
                    StaticParser::new(&RESOURCES, #default_locale);
            }

            /// Necessary to get rustc to re-compile this proc macro if the included sources change.
            #[allow(unused)]
            fn i_depend_on_these_files() {
                #(include_bytes!(#includes);)*
            }
        }
    })
    .into()
}

/// Find all fluent source files from a given root.
/// Returns a vector of (locale, [locale source paths], [locale sources])
fn collect_sources(root: &Path) -> Option<Vec<(String, Vec<String>, Vec<String>)>> {
    assert!(root.is_dir(), "no such directory: {:?}", root);

    let mut result = vec![];
    let mut had_errors = false;

    for child in children(&root) {
        if !child.file_type().unwrap().is_dir() {
            // skip non-subdirectories
            continue;
        }

        let locale = child.file_name().to_string_lossy().to_string();

        let mut locale_paths = vec![];
        let mut locale_sources = vec![];

        for ftl_file in children(&child.path()) {
            if ftl_file
                .path()
                .extension()
                .map(|x| x != "ftl")
                .unwrap_or(false)
            {
                // not an ftl file
                continue;
            }

            let file_source = read_to_string(&ftl_file.path()).expect("failed to read .ftl file");
            let path = ftl_file.path();

            had_errors |= has_errors(
                path.strip_prefix(root).expect("prefix strip failed"),
                file_source.clone(),
            );

            locale_paths.push(path.display().to_string());
            locale_sources.push(file_source);
        }

        if locale_paths.len() == 0 {
            // empty directory
            continue;
        }

        result.push((locale, locale_paths, locale_sources));
    }

    if had_errors {
        None
    } else {
        Some(result)
    }
}

/// If a source has errors, print them and return true.
fn has_errors(path: &Path, source: String) -> bool {
    match fluent_syntax::parser::parse(&source) {
        Ok(_) => false,
        Err((_, errs)) => {
            eprintln!("baked_fluent: parse errors in `{}`", path.display());
            for err in errs {
                error::log_error(path, &source, &err);
            }
            true
        }
    }
}

/// Easily find children of a directory.
fn children(path: &Path) -> impl Iterator<Item = DirEntry> {
    let mut results = path
        .read_dir()
        .expect("no such path")
        .map(|entry| entry.expect("stop changing the filesystem underneath me"))
        .collect::<Vec<_>>();

    results.sort_by_key(DirEntry::file_name); // keep builds deterministic

    results.into_iter()
}
