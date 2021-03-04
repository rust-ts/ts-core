use std::path::{Component, Path, PathBuf};

pub fn normalize_path(path: String) -> String {
  let mut buf = PathBuf::new();
  for component in Path::new(&path).components() {
    match component {
      Component::RootDir => {
        buf.clear();
        buf.push(component);
      }
      Component::ParentDir => {
        buf.pop();
      }
      Component::Normal(s) => {
        buf.push(s);
      }
      _ => (),
    }
  }

  match buf.to_str() {
    Some(s) => s.to_string(),
    None => path,
  }
}

#[inline]
pub fn is_declaration_file(file_name: &str) -> bool {
  file_name.ends_with(".d.ts")
}

#[cfg(test)]
mod tests {
  use super::*;
  use expect_test::{expect, Expect};

  fn expect_equal(paths: &str, expect: Expect) {
    let actual: String = paths
      .split('\n')
      .filter_map(
        |path| if path.is_empty() { None } else { Some(normalize_path(path.to_string())) },
      )
      .collect::<Vec<String>>()
      .join("\n");
    expect.assert_eq(&actual)
  }

  #[test]
  fn test_normalize_path() {
    expect_equal(
      r"
/tmp/foo.txt
C:\tmp\foo.txt
/tmp/../foo/./bar.txt
/tmp/foo/../../bar.txt
",
      expect![[r#"
        /tmp/foo.txt
        C:\tmp\foo.txt
        /foo/bar.txt
        /bar.txt"#]],
    )
  }
}
