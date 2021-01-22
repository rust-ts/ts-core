use crate::{is_line_break, CharacterCodes};

pub trait SourcefileLike {
  fn get_chars(&self) -> &[char];
  fn get_line_starts(&self) -> &[usize];
  fn get_position_of_line_and_character(
    &self,
    line: usize,
    character: usize,
    allow_edits: bool,
  ) -> usize {
    compute_position_of_line_and_character(
      &self.get_line_starts(),
      line,
      character,
      Some(self.get_chars()),
      allow_edits,
    )
  }
}

pub(crate) fn compute_line_starts(chars: &[char]) -> Vec<usize> {
  let mut result = vec![];
  let mut pos = 0;
  let mut line_start = 0;
  while pos < chars.len() {
    let ch = chars[pos];
    pos += 1;

    if ch == CharacterCodes::CarriageReturn && chars[pos] == CharacterCodes::LineFeed {
      pos += 1;
    } else if ch == CharacterCodes::LineFeed
      || ch > CharacterCodes::MaxAsciiCharacter && is_line_break(ch)
    {
      result.push(line_start);
      line_start = pos;
    }
  }
  result.push(line_start);
  result
}

#[inline]
pub(crate) fn get_position_of_line_and_character<S: SourcefileLike>(
  source_file: S,
  line: usize,
  character: usize,
  allow_edits: bool,
) -> usize {
  source_file.get_position_of_line_and_character(line, character, allow_edits)
}

pub(crate) fn compute_position_of_line_and_character(
  line_starts: &[usize],
  line: usize,
  character: usize,
  debug_text: Option<&[char]>,
  allow_edits: bool,
) -> usize {
  let mut line = line;
  let line_len = line_starts.len();
  if line >= line_len {
    if allow_edits {
      // Clamp line to nearest allowable value
      line = if line >= line_len { line_len - 1 } else { line };
    } else {
      panic!(format!(
        "Bad line number. Line: {}, Len(line_starts): {} , line map is correct? {}",
        line,
        line_len,
        if let Some(text) = debug_text {
          (compute_line_starts(text) == line_starts).to_string()
        } else {
          String::from("unknown")
        }
      ));
    }
  }

  let res = line_starts[line] + character;
  if allow_edits {
    // Clamp to nearest allowable values to allow the underlying to be edited without crashing (accuracy is lost, instead)
    // TODO: Somehow track edits between file as it was during the creation of sourcemap we have and the current file and
    // apply them to the computed position to improve accuracy
    return if res > line_starts[line + 1] {
      line_starts[line + 1]
    } else {
      match debug_text {
        Some(text) if res > text.len() => text.len(),
        _ => res,
      }
    };
  }
  if line < line_len - 1 {
    assert!(res < line_starts[line + 1])
  } else if let Some(text) = debug_text {
    assert!(res <= text.len())
  }
  res
}

// /* @internal */
// export function getLineStarts(sourceFile: SourceFileLike): readonly number[] {
//   return sourceFile.lineMap || (sourceFile.lineMap = computeLineStarts(sourceFile.text));
// }

pub struct LineAndCharacter {
  line: usize,
  character: usize,
}

pub(crate) fn compute_line_and_character_of_position(
  line_starts: &[usize],
  position: usize,
) -> LineAndCharacter {
  let line = compute_line_of_position(line_starts, position, None);
  LineAndCharacter {
    line,
    character: position - line_starts[line],
  }
}

pub(crate) fn compute_line_of_position(
  line_starts: &[usize],
  position: usize,
  lower_bound: Option<usize>,
) -> usize {
  match line_starts[lower_bound.unwrap_or(0)..].binary_search(&position) {
    Ok(line_num) => line_num,
    Err(line_num) => {
      assert!(
        line_num != 0,
        "position cannot precede the beginning of the file"
      );
      line_num - 1
    }
  }
}

pub(crate) fn get_lines_between_positions<S: SourcefileLike>(
  source: &S,
  pos1: usize,
  pos2: usize,
) -> usize {
  if pos1 == pos2 {
    0
  } else {
    let line_starts = source.get_line_starts();
    let lower = pos1.min(pos2);
    let is_negative = lower == pos2;
    let upper = if is_negative { pos1 } else { pos2 };
    let lower_line = compute_line_of_position(line_starts, lower, None);
    let upper_line = compute_line_of_position(line_starts, upper, Some(lower_line));
    if is_negative {
      lower_line - upper_line
    } else {
      upper_line - lower_line
    }
  }
}

pub fn get_line_and_character_of_position<S: SourcefileLike>(
  source: S,
  position: usize,
) -> LineAndCharacter {
  compute_line_and_character_of_position(source.get_line_starts(), position)
}
