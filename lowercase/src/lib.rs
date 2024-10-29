#[allow(warnings)]
mod bindings;

use crate::bindings::exports::tnantoka::formatter::formattable::Guest;

struct Component;

impl Guest for Component {
    fn format(text: String) -> String {
        text.to_lowercase()
    }
}

bindings::export!(Component with_types_in bindings);
