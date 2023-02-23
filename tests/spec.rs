use yaml_ed::YamlEd;

use assert_fs::prelude::*;
use indoc::indoc;

#[test]
fn empty_doc() {
    const SOURCE: &'static str = "";

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_doublequotes_oneline() {
    const SOURCE: &'static str = indoc! {r#"a: "1""#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_doublequotes_newline() {
    const SOURCE: &'static str = indoc! {r#"
        a: "1"
        "#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_singlequotes_oneline() {
    const SOURCE: &'static str = indoc! {r#"a: '1'"#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_singlequotes_newline() {
    const SOURCE: &'static str = indoc! {r#"
        a: '1'
        "#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_noquotes_oneline() {
    const SOURCE: &'static str = indoc! {r#"a: 1"#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_noquotes_newline_linux() {
    const SOURCE: &'static str = indoc! {r#"a: 1\nb: 2\n"#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_noquotes_newline_windows() {
    const SOURCE: &'static str = indoc! {r#"a: 1\r\nb: 2\r\n"#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}

#[test]
fn scalar_noquotes_newline_oldmac() {
    const SOURCE: &'static str = indoc! {r#"a: 1\rb: 2\r"#};

    // from string
    let ed = YamlEd::from(SOURCE);
    assert_eq!(ed.to_string(), SOURCE);

    // from file
    let file = assert_fs::NamedTempFile::new("source.yaml").unwrap();
    file.write_str(SOURCE).unwrap();
    let ed = YamlEd::try_from(file.path()).expect("file should be loadable");
    assert_eq!(ed.to_string(), SOURCE);
}
