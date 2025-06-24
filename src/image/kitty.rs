use std::sync::LazyLock;

pub fn has_kitty_support() -> bool {
    *KITTY_SUPPORT
}

static KITTY_SUPPORT: LazyLock<bool> = LazyLock::new(check_kitty_support);

fn check_kitty_support() -> bool {
    if let Ok(term) = std::env::var("TERM") {
        return term.contains("kitty") || term.contains("ghostty");
    }
    false
}
