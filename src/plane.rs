/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Implementation of an editing plane.

use std::fmt;
use std::fmt::Display;

const CH_WS: char = ' ';

/// Checks if the specified character is a box-drawing character.
macro_rules! is_box_drawing_character {
  ($ch:expr) => {
    match $ch {
      '┌' | '┐' | '└' | '┘' | '─' | '│' | '├' | '┤' | '┴' | '┬' | '┼' | '╪' | '╫' | '╬' | '╞' | '╡' | '╥' | '╨' | '═' | '║' | '╟' | '╢' => true,
      _ => false,
    }
  };
}

/// Checks if the specified character is a vertical line seen from the left side.
macro_rules! is_vertical_line_left {
  ($ch:expr) => {
    match $ch {
      '│' | '├' | '║' | '╟' => true,
      _ => false,
    }
  };
}

/// Checks if the specified character is a vertical line seen from the right side.
macro_rules! is_vertical_line_right {
  ($ch:expr) => {
    match $ch {
      '│' | '┤' | '║' | '╢' => true,
      _ => false,
    }
  };
}

/// Checks if the specified character is a crossing with vertical line.
macro_rules! is_vertical_line_crossing {
  ($ch:expr) => {
    match $ch {
      '│' | '┼' | '┬' | '┴' | '╪' | '┐' | '┘' | '├' | '║' | '╟' | '╬' | '╥' | '╨' | '╫' | '╢' | '┤' | '╡' => true,
      _ => false,
    }
  };
}

enum Op {
  Insert,
  Delete,
}

/// Row of characters in columns.
pub struct Row {
  /// Characters in a row.
  pub columns: Vec<char>,
}

impl Display for Row {
  /// Converts a [Row] into its string representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.columns.iter().collect::<String>())
  }
}

impl Row {
  /// Creates a new row with specified characters.
  pub fn new(columns: Vec<char>) -> Self {
    Self { columns }
  }
}

/// Plane containing rows of characters.
pub struct Plane {
  /// Rows in plane.
  rows: Vec<Row>,
  /// Current vertical cursor position (row index).
  row: usize,
  /// Current horizontal cursor position (column index).
  col: usize,
  /// Information item height (0 when not present).
  iih: usize,
}

impl Display for Plane {
  /// Converts a [Plane] into its string representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      self.rows.iter().fold("".to_string(), |plane, row| format!("{}\n{}", plane, row)).trim()
    )
  }
}

impl Plane {
  /// Creates a plane from text.
  pub fn new(content: &str) -> Self {
    let mut rows = vec![];
    for content_line in content.lines() {
      let line = content_line.trim();
      if !line.is_empty() {
        let mut columns = vec![];
        for ch in line.chars() {
          columns.push(ch);
        }
        rows.push(Row::new(columns));
      }
    }
    let iih = information_item_height(&rows);
    Self { rows, row: 1, col: 1, iih }
  }

  /// Returns a reference to rows.
  pub fn rows(&self) -> &[Row] {
    &self.rows
  }

  /// Returns the vertical position of the cursor in plane coordinates.
  pub fn cursor_row(&self) -> usize {
    self.row
  }

  /// Returns the horizontal position of the cursor in plane coordinates.
  pub fn cursor_col(&self) -> usize {
    self.col
  }

  /// Returns `true` if the current cursor position is valid.
  pub fn is_valid_cursor_pos(&self) -> bool {
    (1..self.rows.len() - 1).contains(&self.row) && (1..self.rows[self.row].columns.len() - 1).contains(&self.col)
  }

  /// Moves cursor up.
  pub fn cursor_move_up(&mut self) -> bool {
    if self.is_allowed_position(-1, 0) {
      self.cursor_move(-1, 0);
      return true;
    }
    if self.is_horz_line(-1, 0) && self.is_allowed_position(-2, 0) {
      self.cursor_move(-2, 0);
      return true;
    }
    false
  }

  /// Moves cursor down.
  pub fn cursor_move_down(&mut self) -> bool {
    if self.is_allowed_position(1, 0) {
      self.row += 1;
      return true;
    }
    if self.is_horz_line(1, 0) && self.is_allowed_position(2, 0) {
      self.row += 2;
      return true;
    }
    false
  }

  /// Moves cursor left.
  pub fn cursor_move_left(&mut self) -> bool {
    if self.is_allowed_position(0, -1) {
      self.col -= 1;
      return true;
    }
    if self.is_vert_line(0, -1) && self.is_allowed_position(0, -2) {
      self.col -= 2;
      return true;
    }
    false
  }

  /// Moves cursor right.
  pub fn cursor_move_right(&mut self) -> bool {
    if self.is_allowed_position(0, 1) {
      self.col += 1;
      return true;
    }
    if self.is_vert_line(0, 1) && self.is_allowed_position(0, 2) {
      self.col += 2;
      return true;
    }
    false
  }

  /// Places cursor at the first character in the cell (same row).
  pub fn cursor_move_cell_start(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_left() {
        self.cursor_move(0, offset + 1);
        return true;
      }
    }
    false
  }

  /// Places cursor at the last character in the cell (same row).
  pub fn cursor_move_cell_end(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_right() {
        self.cursor_move(0, offset - 1);
        return true;
      }
    }
    false
  }

  /// Places cursor at the first character in the decision table (same row).
  pub fn cursor_move_table_start(&mut self) -> bool {
    if (1..self.rows.len() - 1).contains(&self.row) {
      return if is_box_drawing_character!(self.rows[self.row].columns[1]) {
        self.cursor_move_cell_start()
      } else {
        self.col = 1;
        true
      };
    }
    false
  }

  /// Places cursor at the last character in the cell (same row).
  pub fn cursor_move_table_end(&mut self) -> bool {
    if (1..self.rows.len() - 1).contains(&self.row) {
      let index = self.rows[self.row].columns.len() - 2;
      return if is_box_drawing_character!(self.rows[self.row].columns[index]) {
        self.cursor_move_cell_end()
      } else {
        self.col = self.rows[self.row].columns.len() - 2;
        true
      };
    }
    false
  }

  /// Places cursor at the first character in the next cell to the right.
  pub fn cursor_move_cell_right(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_right() {
        return if self.is_allowed_position(0, offset + 1) {
          self.cursor_move(0, offset + 1);
          true
        } else {
          self.cursor_move_cell_end()
        };
      }
    }
    false
  }

  /// Places cursor at the last character in the next cell to the left.
  pub fn cursor_move_cell_left(&mut self) -> bool {
    if self.is_valid_cursor_pos() {
      if let Some(offset) = self.get_vert_line_offset_left() {
        return if self.is_allowed_position(0, offset - 1) {
          self.cursor_move(0, offset - 1);
          true
        } else {
          self.cursor_move_cell_start()
        };
      }
    }
    false
  }

  /// Inserts a character at the current position.
  pub fn insert_char(&mut self, ch: char) {
    if self.is_valid_cursor_pos() {
      let pos = self.last_position_before_vert_line();
      let (found, offset) = self.is_whitespace_before_vert_line();
      let columns = &mut self.rows[self.row].columns;
      columns.insert(self.col, ch);
      if found {
        columns.remove(self.col + offset + 1);
      } else {
        self.insert_column_before_vert_line(pos);
      }
      self.cursor_move(0, 1);
      self.update_joins();
    }
  }

  /// Deletes a character placed *before* the cursor.
  pub fn delete_char_before(&mut self) {
    if self.is_allowed_position(0, -1) {
      let pos = self.last_position_before_vert_line();
      self.rows[self.row].columns.insert(pos + 1, CH_WS);
      self.rows[self.row].columns.remove(self.col - 1);
      if self.is_whitespace_column_before_vert_line(pos, Op::Delete) {
        self.delete_column_before_vert_line(pos);
      }
      self.cursor_move(0, -1);
      self.update_joins();
    }
  }

  /// Deletes a character placed *under* the cursor.
  pub fn delete_char(&mut self) {
    let pos = self.last_position_before_vert_line();
    self.rows[self.row].columns.insert(pos + 1, CH_WS);
    self.rows[self.row].columns.remove(self.col);
    if self.is_whitespace_column_before_vert_line(pos, Op::Delete) {
      self.delete_column_before_vert_line(pos);
    }
    if is_box_drawing_character!(self.rows[self.row].columns[self.col]) {
      self.cursor_move(0, -1);
    }
    self.update_joins();
  }

  /// Moves the cursor to new position.
  fn cursor_move(&mut self, row_offset: i32, col_offset: i32) {
    if self.is_allowed_position(row_offset, col_offset) {
      let (row, col) = self.adjusted_position(row_offset, col_offset);
      if (1..self.rows.len() - 1).contains(&row) && (1..self.rows[row].columns.len() - 1).contains(&col) {
        self.row = row;
        self.col = col;
      }
    }
  }

  /// Updated join character between information item name cell and the body of the decision table.
  fn update_joins(&mut self) {
    if self.iih > 0 {
      let row_index = self.iih;
      // remove old joining character...
      for ch in &mut self.rows[row_index].columns {
        match ch {
          '┴' => *ch = '─',
          '┼' => *ch = '┬',
          '┤' => *ch = '┐',
          _ => {}
        }
      }
      // ...and replace with new joining character
      let col_index = self.rows[0].columns.len() - 1;
      if col_index < self.rows[row_index].columns.len() {
        let ch = &mut self.rows[row_index].columns[col_index];
        match ch {
          '─' => *ch = '┴',
          '┬' => *ch = '┼',
          '┐' => *ch = '┤',
          _ => {}
        }
      }
    }
  }

  /// Returns the position of the last character before the vertical line.
  fn last_position_before_vert_line(&self) -> usize {
    for (pos, ch) in self.rows[self.row].columns.iter().enumerate().skip(self.col) {
      if is_vertical_line_left!(ch) {
        return pos - 1;
      }
    }
    self.col
  }

  ///
  fn is_whitespace_before_vert_line(&self) -> (bool, usize) {
    let mut count = 0;
    let mut offset = 0;
    for ch in &self.rows[self.row].columns[self.col + 1..] {
      if is_vertical_line_left!(ch) {
        break;
      } else if *ch == CH_WS {
        count += 1;
      } else {
        count = 0;
      }
      offset += 1;
    }
    (count > 0, offset)
  }

  ///
  fn insert_column_before_vert_line(&mut self, pos: usize) {
    let (skip, take) = self.rows_skip_and_take(Op::Insert);
    for (row_index, row) in self.rows.iter_mut().enumerate().skip(skip).take(take) {
      if row_index != self.row && pos < row.columns.len() - 1 {
        let mut found_char = CH_WS;
        let mut found_index = 0;
        for (col_index, ch) in row.columns[pos..].iter().enumerate() {
          if is_vertical_line_crossing!(ch) {
            found_char = *ch;
            found_index = pos + col_index;
            break;
          }
        }
        match found_char {
          '│' | '├' | '║' | '╟' => row.columns.insert(found_index, CH_WS),
          '┼' | '┬' | '┴' | '┐' | '┘' | '┤' | '╥' | '╨' | '╫' | '╢' => row.columns.insert(found_index, '─'),
          '╪' | '╬' | '╡' => row.columns.insert(found_index, '═'),
          _ => {}
        }
      }
    }
  }

  /// Returns `true` if there is a whitespace is before the next vertical line
  /// to the right from the specified position in each checked row.
  fn is_whitespace_column_before_vert_line(&self, pos: usize, op: Op) -> bool {
    let (skip, take) = self.rows_skip_and_take(op);
    for (row_index, row) in self.rows.iter().enumerate().skip(skip).take(take) {
      // check if the current column is not after the end of each row
      if (1..row.columns.len() - 1).contains(&pos) {
        // check the character at column position, if box-drawing then skip
        let ch = self.rows[row_index].columns[pos];
        if !is_box_drawing_character!(ch) {
          // move to the right until vertical line is found
          for chars in row.columns[pos - 1..].windows(3) {
            if is_vertical_line_left!(chars[2]) {
              // if there is no whitespace before vertical line,
              // no further checking is needed, just return `false`
              if chars[1] != CH_WS {
                return false;
              }
              // if there is a whitespace, but just between two box-drawing
              // characters, no further checking is needed, just return `false`
              if is_box_drawing_character!(chars[0]) {
                return false;
              }
              // whitespace found, check the next row
              break;
            }
          }
        }
      }
    }
    true
  }

  /// Deletes a single character before the next vertical line to the right
  /// from the specified position.
  fn delete_column_before_vert_line(&mut self, pos: usize) {
    let (skip, take) = self.rows_skip_and_take(Op::Delete);
    for row in self.rows.iter_mut().skip(skip).take(take) {
      if pos < row.columns.len() - 1 {
        let mut found_index = 0;
        for (col_index, ch) in row.columns[pos..].iter().enumerate() {
          if is_vertical_line_crossing!(ch) {
            found_index = pos + col_index;
            break;
          }
        }
        if found_index > 0 {
          row.columns.remove(found_index - 1);
        }
      }
    }
  }

  /// Returns `true` when the character at the specified position is a horizontal line.
  fn is_horz_line(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r < self.rows.len() && c < self.rows[r].columns.len() {
      matches!(self.rows[r].columns[c], '─' | '═')
    } else {
      false
    }
  }

  /// Returns `true` when the character at the specified position is a vertical line.
  fn is_vert_line(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r < self.rows.len() && c < self.rows[r].columns.len() {
      matches!(self.rows[r].columns[c], '│' | '║')
    } else {
      false
    }
  }

  /// Returns `true` when the cursor position is allowed according to horizontal and vertical offset.
  fn is_allowed_position(&self, row_offset: i32, col_offset: i32) -> bool {
    let (r, c) = self.adjusted_position(row_offset, col_offset);
    if r > 0 && r < self.rows.len() - 1 && c > 0 && c < self.rows[r].columns.len() - 1 {
      !is_box_drawing_character!(self.rows[r].columns[c])
    } else {
      false
    }
  }

  /// Calculates a new position according the specified row and column offset.
  fn adjusted_position(&self, row_offset: i32, col_offset: i32) -> (usize, usize) {
    (
      if row_offset >= 0 {
        self.row.saturating_add(row_offset as usize)
      } else {
        self.row.saturating_sub(row_offset.unsigned_abs() as usize)
      },
      if col_offset >= 0 {
        self.col.saturating_add(col_offset as usize)
      } else {
        self.col.saturating_sub(col_offset.unsigned_abs() as usize)
      },
    )
  }

  /// Returns the offset of the vertical line to the right from current cursor position.
  fn get_vert_line_offset_right(&self) -> Option<i32> {
    if self.is_valid_cursor_pos() {
      for (index, ch) in self.rows[self.row].columns[self.col..].iter().enumerate() {
        if is_vertical_line_left!(ch) {
          if let Ok(offset) = index.try_into() {
            return Some(offset);
          }
        }
      }
    }
    None
  }

  /// Returns the offset of the vertical line to the left from current cursor position.
  fn get_vert_line_offset_left(&self) -> Option<i32> {
    if self.is_valid_cursor_pos() {
      for (offset, ch) in self.rows[self.row].columns[0..=self.col].iter().rev().enumerate() {
        if is_vertical_line_right!(ch) {
          return Some(-(offset as i32));
        }
      }
    }
    None
  }

  /// Returns the number of rows to skip and to take while iterating over rows.
  fn rows_skip_and_take(&self, op: Op) -> (usize, usize) {
    if self.row < self.iih {
      // operation takes place in information item cell
      match op {
        Op::Insert => {
          //
          let pos = self.last_position_before_vert_line();
          if pos + 1 >= self.rows[self.iih].columns.len() {
            (0, self.rows.len())
          } else {
            (0, self.iih)
          }
        }
        Op::Delete => {
          //
          (0, self.iih)
        }
      }
    } else {
      // operation takes place in decision table body
      match op {
        Op::Insert => {
          //
          (self.iih, self.rows.len() - self.iih)
        }
        Op::Delete => {
          //
          let l_first = self.rows[0].columns.len();
          let l_current = self.rows[self.row].columns.len();
          if l_current > l_first {
            (self.iih, self.rows.len() - self.iih)
          } else {
            (0, self.rows.len())
          }
        }
      }
    }
  }
}

/// Calculates the height of the information item cell at the beginning of the decision table.
fn information_item_height(rows: &[Row]) -> usize {
  for (row_index, row) in rows.iter().enumerate() {
    for (col_index, ch) in row.columns.iter().enumerate() {
      if col_index == 0 && *ch != '┌' && *ch != '├' {
        // skip rows that do not begin with horizontal line crossing
        break;
      }
      if *ch == '╥' {
        // index of the row that contains '╥' character is the height
        return row_index;
      }
    }
  }
  0
}
