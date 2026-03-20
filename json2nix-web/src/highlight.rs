use std::sync::LazyLock;

use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

const CODE_HIGHLIGHTING_CSS_CLASS_PREFIX: &'static str = "j2n-";

type SyntaxData = (SyntaxReference, SyntaxSet, ClassStyle);

static SYNTAX_DATA: LazyLock<SyntaxData> = LazyLock::new(|| {
    let syntax_set = two_face::syntax::extra_newlines();
    let syntax = syntax_set.find_syntax_by_name("Nix").expect("Nix syntax to be provided by two_face");

    let class_style = ClassStyle::SpacedPrefixed {
        prefix: CODE_HIGHLIGHTING_CSS_CLASS_PREFIX,
    };

    (syntax.clone(), syntax_set, class_style)
});

pub(crate) fn highlight_nix_code(code: &str) -> Result<String, String> {
    let mut nix_html_generator = ClassedHTMLGenerator::new_with_class_style(&SYNTAX_DATA.0, &SYNTAX_DATA.1, SYNTAX_DATA.2);

    for line in LinesWithEndings::from(code) {
        if let Err(err) = nix_html_generator.parse_html_for_line_which_includes_newline(line) {
            return Err(format!("Failed to parse Nix code for highlighting: {}", err));
        }
    }

    Ok(nix_html_generator.finalize())
}
