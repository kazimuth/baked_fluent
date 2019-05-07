//! Code for pretty-printing fluent source errors.
//! Based on https://github.com/projectfluent/fluent-rs/blob/master/fluent-cli/src/main.rs

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use fluent_syntax::parser::errors::{ErrorKind, ParserError};
use std::cmp;
use std::path::Path;

/// Log an error in pretty rustc-style
pub fn log_error(path: &Path, source: &str, err: &ParserError) {
    let slice = if let Some(slice) = err.slice {
        slice
    } else {
        (err.pos.0, err.pos.1)
    };

    eprintln!(
        "{:?} {} {}",
        path,
        path.display(),
        path.display().to_string()
    );

    let (id, desc) = get_error_info(&err.kind);
    let end_pos = cmp::min(err.pos.1, slice.1);
    let snippet = Snippet {
        slices: vec![Slice {
            source: source[slice.0..slice.1].to_string(),
            line_start: get_line_num(&source, err.pos.0) + 1,
            origin: Some(path.display().to_string()),
            fold: false,
            annotations: vec![SourceAnnotation {
                label: desc.to_string(),
                annotation_type: AnnotationType::Error,
                range: (err.pos.0 - slice.0, end_pos - slice.0 + 1),
            }],
        }],
        title: Some(Annotation {
            label: Some(desc.to_string()),
            id: Some(id.to_string()),
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
    };
    let dl = DisplayList::from(snippet);
    let dlf = DisplayListFormatter::new(true);
    eprintln!("{}", dlf.format(&dl));
    eprintln!("-----------------------------");
}

fn get_line_num(source: &str, pos: usize) -> usize {
    let mut ptr = 0;
    let mut i = 0;

    let lines = source.lines();

    for line in lines {
        let lnlen = line.chars().count();
        ptr += lnlen + 1;

        if ptr > pos {
            break;
        }
        i += 1;
    }

    i
}

fn get_error_info(kind: &ErrorKind) -> (&'static str, String) {
    // TODO: are these error codes defined somewhere?
    match kind {
        &ErrorKind::Generic => ("E0001", "Generic error".to_string()),
        &ErrorKind::ExpectedEntry => ("E0002", "Expected an entry start".to_string()),
        &ErrorKind::ExpectedToken(ch) => ("E0003", format!("Expected token: \"{}\"", ch)),
        &ErrorKind::ExpectedCharRange { ref range } => (
            "E0004",
            format!("Expected a character from range: \"{}\"", range),
        ),
        &ErrorKind::ExpectedMessageField { ref entry_id } => (
            "E0005",
            format!(
                "Expected message \"{}\" to have a value or attributes",
                entry_id
            ),
        ),
        &ErrorKind::ExpectedTermField { ref entry_id } => (
            "E0006",
            format!("Expected term \"{}\" to have a value", entry_id),
        ),
        &ErrorKind::ForbiddenWhitespace => {
            ("E0007", "Keyword cannot end with a whitespace".to_string())
        }
        &ErrorKind::ForbiddenCallee => (
            "E0008",
            "The callee has to be a simple, upper-case identifier".to_string(),
        ),
        &ErrorKind::ForbiddenKey => ("E0009", "The key has to be a simple identifier".to_string()),
        &ErrorKind::MissingDefaultVariant => (
            "E0010",
            "Expected one of the variants to be marked as default **)".to_string(),
        ),
        &ErrorKind::MissingValue => ("E0012", "Expected value.".to_string()),
        &ErrorKind::TermAttributeAsPlaceable => (
            "E0019",
            "Attributes of terms cannot be used as placeables".to_string(),
        ),
        &ErrorKind::MissingVariantKey => ("E0021", "Missing variant key".to_string()),
        &ErrorKind::MissingLiteral => ("E0022", "Missing literal".to_string()),
        &ErrorKind::MultipleDefaultVariants => (
            "E0023",
            "Expression cannot have multiple default variants".to_string(),
        ),
        &ErrorKind::MessageReferenceAsSelector => (
            "E0024",
            "Message reference cannot be used as selector".to_string(),
        ),
        &ErrorKind::TermReferenceAsSelector => (
            "E0025",
            "Term reference cannot be used as selector".to_string(),
        ),
        &ErrorKind::MessageAttributeAsSelector => (
            "E0026",
            "Message attribute cannot be used as selector".to_string(),
        ),
        &ErrorKind::UnterminatedStringExpression => {
            ("E0028", "Unterminated string expression".to_string())
        }
        &ErrorKind::PositionalArgumentFollowsNamed => (
            "E0029",
            "Positional argument follows named argument".to_string(),
        ),
        &ErrorKind::DuplicatedNamedArgument(ref arg) => {
            ("E0030", format!("Duplicated named argument `{}`", arg))
        }
        &ErrorKind::ForbiddenVariantAccessor => {
            ("E0031", "Forbidden variant accessor.".to_string())
        }
        &ErrorKind::UnknownEscapeSequence(ref seq) => {
            ("E0032", format!("Unknown escape sequence {:?}", seq))
        }
        &ErrorKind::InvalidUnicodeEscapeSequence(ref seq) => {
            ("E0033", format!("Invalid escape sequence {:?}", seq))
        }
        &ErrorKind::UnbalancedClosingBrace => ("E0034", "Unbalanced closing brace".to_string()),
        &ErrorKind::ExpectedInlineExpression => ("E0035", "Expected inline expression".to_string()),
        kind => ("E0000", format!("Other error: {:?}", kind)),
    }
}
