// FIXME: Add "run-rustfix" once it's supported for multipart suggestions
// aux-build:option_helpers.rs

#![warn(clippy::option_map_unwrap_or, clippy::option_map_unwrap_or_else)]

#[macro_use]
extern crate option_helpers;

use std::collections::HashMap;

/// Checks implementation of the following lints:
/// * `OPTION_MAP_UNWRAP_OR`
/// * `OPTION_MAP_UNWRAP_OR_ELSE`
#[rustfmt::skip]
fn option_methods() {
    let opt = Some(1);

    // Check `OPTION_MAP_UNWRAP_OR`.
    // Single line case.
    let _ = opt.map(|x| x + 1)
        // Should lint even though this call is on a separate line.
        .unwrap_or(0);
    // Multi-line cases.
    let _ = opt.map(|x| {
        x + 1
    }
    ).unwrap_or(0);
    let _ = opt.map(|x| x + 1)
        .unwrap_or({
            0
        });
    // Single line `map(f).unwrap_or(None)` case.
    let _ = opt.map(|x| Some(x + 1)).unwrap_or(None);
    // Multi-line `map(f).unwrap_or(None)` cases.
    let _ = opt.map(|x| {
        Some(x + 1)
    }
    ).unwrap_or(None);
    let _ = opt
        .map(|x| Some(x + 1))
        .unwrap_or(None);
    // macro case
    let _ = opt_map!(opt, |x| x + 1).unwrap_or(0); // should not lint

    // Should not lint if not copyable
    let id: String = "identifier".to_string();
    let _ = Some("prefix").map(|p| format!("{}.{}", p, id)).unwrap_or(id);
    // ...but DO lint if the `unwrap_or` argument is not used in the `map`
    let id: String = "identifier".to_string();
    let _ = Some("prefix").map(|p| format!("{}.", p)).unwrap_or(id);

    // Check OPTION_MAP_UNWRAP_OR_ELSE
    // single line case
    let _ = opt.map(|x| x + 1)
        // Should lint even though this call is on a separate line.
        .unwrap_or_else(|| 0);
    // Multi-line cases.
    let _ = opt.map(|x| {
        x + 1
    }
    ).unwrap_or_else(|| 0);
    let _ = opt.map(|x| x + 1)
        .unwrap_or_else(||
            0
        );
    // Macro case.
    // Should not lint.
    let _ = opt_map!(opt, |x| x + 1).unwrap_or_else(|| 0);

    // Issue #4144
    {
        let mut frequencies = HashMap::new();
        let word = "foo";

        frequencies
            .get_mut(word)
            .map(|count| {
                *count += 1;
            })
            .unwrap_or_else(|| {
                frequencies.insert(word.to_owned(), 1);
            });
    }
}

fn main() {
    option_methods();
}
