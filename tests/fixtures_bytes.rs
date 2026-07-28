use rstest::{fixture, rstest};
use std::fs;
use std::path::{Path, PathBuf};

const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

// Fixtures whose bytes deviate on purpose. Everything else is plain LF,
// no BOM, final newline present.
const CRLF: [&str; 2] = ["syntax/crlf.json", "pairs/crlf-base/base.json"];
const BOM_FILE: &str = "syntax/bom.json";
const NO_FINAL_NEWLINE: &str = "syntax/no-final-newline.json";
const EMPTY: &str = "invalid/empty.json";

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn relative(path: &Path) -> String {
    path.strip_prefix(corpus())
        .expect("fixture outside the corpus")
        .to_string_lossy()
        .replace('\\', "/")
}

fn bytes_of(rel: &str) -> Vec<u8> {
    let path = corpus().join(rel);
    fs::read(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn text_of(rel: &str) -> String {
    String::from_utf8(bytes_of(rel)).unwrap_or_else(|e| panic!("{rel} is not utf-8: {e}"))
}

#[fixture]
fn escapes() -> String {
    text_of("syntax/escapes.json")
}

#[fixture]
fn numbers() -> String {
    text_of("syntax/numbers.json")
}

#[fixture]
fn unicode() -> String {
    text_of("syntax/unicode.json")
}

#[rstest]
fn byte_quirks_stay_in_their_own_fixture(#[files("tests/fixtures/**/*.json")] path: PathBuf) {
    let rel = relative(&path);
    let bytes = fs::read(&path).expect("unreadable fixture");

    assert_eq!(
        bytes.contains(&b'\r'),
        CRLF.contains(&rel.as_str()),
        "{rel}: CR"
    );
    assert_eq!(bytes.starts_with(BOM), rel == BOM_FILE, "{rel}: BOM");

    if rel == EMPTY {
        assert!(bytes.is_empty(), "{rel} must stay zero-length");
    } else {
        assert_eq!(
            bytes.last() == Some(&b'\n'),
            rel != NO_FINAL_NEWLINE,
            "{rel}: final newline"
        );
    }
}

#[rstest]
#[case(CRLF[0])]
#[case(CRLF[1])]
fn crlf_fixture_has_no_bare_line_feed(#[case] rel: &str) {
    let bytes = bytes_of(rel);
    let bare = bytes
        .iter()
        .enumerate()
        .filter(|(i, b)| **b == b'\n' && (*i == 0 || bytes[i - 1] != b'\r'))
        .count();
    assert_eq!(
        bare, 0,
        "{rel}: {bare} line feeds lost their carriage return"
    );
}

// A round-trip test cannot replace these:
// a parser and a printer that both expand escaping agree with each other,
// stay green, and leave the fixture proving nothing.
#[rstest]
#[case("\\u0041")]
#[case("\\u00e9")]
#[case("\\u4e2d")]
#[case("\\ud83d")]
#[case("\\ude80")]
#[case("\\\"")]
#[case("\\\\")]
#[case("\\/")]
#[case("\\t")]
#[case("\\n")]
#[case("\\r")]
#[case("\\b")]
#[case("\\f")]
fn escape_sequence_survives(escapes: String, #[case] escape: &str) {
    assert!(escapes.contains(escape), "escapes.json lost {escape}");
}

#[rstest]
#[case("1e3")]
#[case("1E+3")]
#[case("1.5E-7")]
#[case("-0.0")]
#[case("0.1000")]
#[case("12345678901234567890")]
fn number_token_stays_raw(numbers: String, #[case] token: &str) {
    assert!(numbers.contains(token), "numbers.json rewrote {token}");
}

// Both render as "e with acute" and differ byte for byte.
#[rstest]
#[case('\u{0301}')]
#[case('\u{00e9}')]
fn normalization_form_survives(unicode: String, #[case] expected: char) {
    assert!(unicode.contains(expected), "unicode.json lost {expected:?}");
}

#[rstest]
#[case("syntax/indent-tab.json")]
#[case("syntax/mixed-style.json")]
fn indentation_keeps_tabs(#[case] rel: &str) {
    assert!(text_of(rel).contains('\t'), "{rel} lost its tabs");
}
