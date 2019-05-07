#![recursion_limit = "128"]

//! Internationalization codegen.

extern crate proc_macro;

use quote::quote;
use std::env;
use std::fs::{read_to_string, DirEntry};
use std::path::{Path, PathBuf};

mod input;

/// The impl_localize macro.
#[proc_macro]
pub fn impl_localize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse input.
    let ast = syn::parse_macro_input!(item as input::ImplLocalize);

    // load all source files.
    let mut root = PathBuf::from(
        &env::var("CARGO_MANIFEST_DIR").expect("baked_fluent doesn't work without cargo"),
    );
    root.push(&ast.path);
    let sources = collect_sources(&root);

    if sources.len() == 0 {
        eprintln!("banked_fluent warning: no fluent .ftl translation files provided in i18n directory, localize() won't do much");
    }

    if let None = sources
        .iter()
        .find(|(locale, _, _)| locale == &ast.default_locale)
    {
        panic!("baked_fluent error: no translations for default locale");
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
                        .into_boxed_slice()
                )
            }

            #[inline]
            fn localize(&self,
                message: &str,
                args: &[(&str, &::baked_fluent::runtime::I18nValue)])
                    -> ::baked_fluent::Result<String> {
                    __i18n_hidden::STATIC_PARSER.localize(&self.0, message, args)
            }

            fn has_message(&self, message: &str) -> bool {
                __i18n_hidden::STATIC_PARSER.has_message(&self.0, message)
            }

            fn default_locale() -> &'static str {
                #default_locale
            }
        }

        #[doc(hidden)]
        mod __i18n_hidden {
            use ::baked_fluent::runtime::{
                StaticParser, Resources,
                Sources, lazy_static, I18nValue
            };

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
/// Panics if any errors are present in fluent sources.
fn collect_sources(root: &Path) -> Vec<(String, Vec<String>, Vec<String>)> {
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

            had_errors |= has_errors(&path, file_source.clone());

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
        panic!("baked_fluent error: fluent source files have errors, not continuing")
    }

    result
}

/// If a source has errors, print them and return true.
fn has_errors(path: &Path, source: String) -> bool {
    match fluent_bundle::FluentResource::try_new(source.clone()) {
        Ok(_) => false,
        Err((_, errs)) => {
            eprintln!(
                "baked_fluent error: fluent parse errors in `{}`",
                path.display()
            );
            for err in errs {
                let (line, col) = linecol(&source, err.pos.0);
                eprintln!(
                    "baked_fluent error:     {}:{}:{}: {:?}",
                    path.display(),
                    line,
                    col,
                    err.kind
                );
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

/// Given a source string and an offset, return a line and column (both 1-based.)
fn linecol(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in src.chars().enumerate() {
        if i == offset {
            return (line, col);
        }

        col += 1;
        if c == '\n' {
            col = 0;
            line += 1;
        }
    }

    (line, col)
}
