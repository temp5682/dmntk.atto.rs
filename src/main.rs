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

//! Implementation of a minimalistic decision table editor.

extern crate ncurses;

mod keys;
mod plane;

use keys::*;
use ncurses::*;
use plane::*;
use std::{env, fs};

/// Initializes ncurses.
fn initialize() {
  // set locale to Unicode en-US
  let locale_conf = LcCategory::all;
  setlocale(locale_conf, "en_US.UTF-8");
  // start ncurses
  initscr();
  // switch to raw mode
  raw();
  // allow for extended keyboard (like F1)
  keypad(stdscr(), true);
  // disable echo
  noecho();
}

/// Terminates ncurses.
fn finalize() {
  // terminate ncurses
  endwin();
}

/// Loads the input file.
fn load_from_file(file_name: &str) -> Plane {
  let content = fs::read_to_string(file_name).unwrap_or_else(|_| panic!("Loading input file failed: {}", file_name));
  Plane::new(&content)
}

/// Processes input keystrokes.
fn process_keystrokes(plane: &mut Plane) {
  let mut cur_x = 0;
  let mut cur_y = 0;
  for row in &plane.rows {
    mv(cur_y, cur_x);
    addstr(&row.to_string());
    cur_y += 1;
  }
  cur_x = plane.cur_screen_x();
  cur_y = plane.cur_screen_y();
  mv(cur_y, cur_x);
  loop {
    let ch = getch();
    let key_name = keyname(ch).unwrap_or_default();
    match key_name.as_str() {
      KEY_NAME_EXIT => break,
      KEY_NAME_UP => {
        if plane.move_up() {
          mv(plane.cur_screen_y(), plane.cur_screen_x());
        }
      }
      KEY_NAME_DOWN => {
        if plane.move_down() {
          mv(plane.cur_screen_y(), plane.cur_screen_x());
        }
      }
      KEY_NAME_LEFT => {
        if plane.move_left() {
          mv(plane.cur_screen_y(), plane.cur_screen_x());
        }
      }
      KEY_NAME_RIGHT => {
        if plane.move_right() {
          mv(plane.cur_screen_y(), plane.cur_screen_x());
        }
      }
      _ => {
        getyx(stdscr(), &mut cur_y, &mut cur_x);
        mvaddstr(40, 1, &format!("{:X}", ch));
        mvaddstr(41, 1, &format!("{:40}", key_name));
        mv(cur_y, cur_x);
        refresh();
      }
    }
  }
}

/// Prints usage message.
fn usage() {
  println!("usage")
}

/// Main entrypoint.
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    usage();
    return;
  }
  initialize();
  let mut plane = load_from_file(&args[1]);
  process_keystrokes(&mut plane);
  finalize();
}