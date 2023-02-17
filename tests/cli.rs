use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn print_file() -> Result<(), Box<dyn std::error::Error>> {
    const CONTENT: &'static str = indoc! {r#"
        a: 1
        b:
          bb: 2
        c:
          cc:
            ccc: 3
        "#};
    let file = assert_fs::NamedTempFile::new("sample.yaml")?;
    file.write_str(CONTENT)?;

    let mut cmd = Command::cargo_bin("yaml-ed")?;
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(CONTENT));

    Ok(())
}
