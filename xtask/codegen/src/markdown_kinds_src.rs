use crate::kind_src::KindsSrc;

pub const MARKDOWN_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("/", "SLASH"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("**", "DOUBLE_STAR"),
        ("`", "BACKTICK"),
        ("```", "TRIPLE_BACKTICK"),
        ("~", "TILDE"),
        ("   ", "WHITESPACE3"),
        ("_", "UNDERSCORE"),
        ("__", "DOUBLE_UNDERSCORE"),
        ("#", "HASH"),
        (",", "COMMA"),
    ],
    keywords: &["null"],
    literals: &[
        "MD_HARD_LINE_LITERAL",
        "MD_SOFT_BREAK_LITERAL",
        "MD_TEXTUAL_LITERAL",
        "MD_STRING_LITERAL",
        "MD_INDENT_CHUNK_LITERAL",
        "MD_THEMATIC_BREAK_LITERAL",
        "MD_ERROR_LITERAL",
    ],
    tokens: &["ERROR_TOKEN", "NEWLINE", "WHITESPACE", "TAB"],
    nodes: &[
        // Bogus nodes
        "BOGUS",
        "MD_BOGUS",
        // node
        "MD_DOCUMENT",
        "MD_BLOCK_LIST",
        "MD_HASH_LIST",
        "MD_HASH",
        "MD_HEADER",
        "MD_INDENT_CODE_BLOCK",
        "MD_FENCED_CODE_BLOCK",
        "MD_CODE_NAME_LIST",
        "MD_HTML_BLOCK",
        "MD_LINK_BLOCK",
        "MD_QUOTE",
        "MD_ORDER_LIST_ITEM",
        "MD_BULLET_LIST_ITEM",
        "MD_BULLET_LIST",
        "MD_ORDER_LIST",
        "MD_PARAGRAPH",
        "MD_INLINE_ITEM_LIST",
        "MD_INLINE_EMPHASIS",
        "MD_INLINE_ITALIC",
        "MD_INLINE_CODE",
        "MD_BULLET",
        "MD_INLINE_LINK",
        "MD_INLINE_IMAGE",
        "MD_INLINE_IMAGE_ALT",
        "MD_INDENTED_CODE_LINE",
        "MD_INLINE_IMAGE_LINK",
        "MD_INLINE_IMAGE_SOURCE",
        "MD_INDENTED_CODE_LINE_LIST",
        "MD_HARD_LINE",
        "MD_SOFT_BREAK",
        "MD_TEXTUAL",
        "MD_SETEXT_HEADER",
        "MD_STRING",
        "MD_INDENT",
        "MD_THEMATIC_BREAK_BLOCK",
    ],
};
