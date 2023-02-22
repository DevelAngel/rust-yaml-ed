use yaml_ed::YamlEd;

use indoc::indoc;

#[test]
fn empty_doc() {
    const SOURCE: &'static str = "";
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_doublequotes_oneline() {
    const SOURCE: &'static str = indoc! {r#"a: "1""#};
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_doublequotes_newline() {
    const SOURCE: &'static str = indoc! {r#"
        a: "1"
        "#};
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_singlequotes_oneline() {
    const SOURCE: &'static str = indoc! {r#"a: '1'"#};
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_singlequotes_newline() {
    const SOURCE: &'static str = indoc! {r#"
        a: '1'
        "#};
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_noquotes_oneline() {
    const SOURCE: &'static str = indoc! {r#"a: 1"#};
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_noquotes_newline() {
    const SOURCE: &'static str = indoc! {r#"
        a: 1
        "#};
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);
}
