use super::*;
use crate::Plane;

const EXPECTED_0001: &str = r#"
┌────────────────────────────────────┐
│ Order options                      │
├──┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U│           │       ║    Order options    ║             │           │
│  │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
│  │   type    │ size  ║ Discount │ Priority ║             │           │
│  ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│  │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│  │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│  │           │       ║   0.05   │ "Low"    ║             │           │
╞══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1│"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├──┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2│"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├──┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3│"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└──┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0001() {
  let plane = &mut Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_col());
  assert_eq!(1, plane.cur_screen_row());
  plane.move_cursor(3, 2);
  plane.delete_character_before();
  eq(EXPECTED_0001, plane);
}

const EXPECTED_0002: &str = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Custome   │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type    │ size  ║ Discount │ Priority ║             │           │
│   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │           │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0002() {
  let plane = &mut Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_col());
  assert_eq!(1, plane.cur_screen_row());
  plane.move_cursor(3, 13);
  plane.delete_character_before();
  eq(EXPECTED_0002, plane);
}

const EXPECTED_0003: &str = r#"
┌────────────────────────────────────┐
│ Order options                      │
├───┬──────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │          │       ║    Order options    ║             │           │
│   │ Customer │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type   │ size  ║ Discount │ Priority ║             │           │
│   ├──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Busine,  │  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private" │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │          │       ║   0.05   │ "Low"    ║             │           │
╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business"│ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private" │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0003() {
  let plane = &mut Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_col());
  assert_eq!(1, plane.cur_screen_row());
  plane.move_cursor(6, 14);
  repeat(3, || {
    plane.delete_character_before();
  });
  eq(EXPECTED_0003, plane);
}

const EXPECTED_0004: &str = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Cust      │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type    │ size  ║ Discount │ Priority ║             │           │
│   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │           │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0004() {
  let plane = &mut Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_col());
  assert_eq!(1, plane.cur_screen_row());
  plane.move_cursor(3, 9);
  repeat(4, || {
    plane.delete_character();
  });
  eq(EXPECTED_0004, plane);
}

const EXPECTED_0005: &str = r#"
┌────────────────────────────────────┐
│ Order options                      │
├───┬──────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │          │       ║    Order options    ║             │           │
│   │ Customer │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type   │ size  ║ Discount │ Priority ║             │           │
│   ├──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business"│  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private" │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │          │       ║   0.05   │ "Low"    ║             │           │
╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business"│ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private" │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0005() {
  let plane = &mut Plane::new(EX_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(6, 14);
  plane.delete_character();
  eq_cursor(7, 14, plane);
  eq(EXPECTED_0005, plane);
}

const EXPECTED_0006: &str = r#"
┌───────────────────────────┐
│ Order options             │
├───┬─┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │ │       ║    Order options    ║             │           │
│   │ │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │ │ size  ║ Discount │ Priority ║             │           │
│   ├─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │ │  <10, ║   0.10,  │"Normal", ║             │           │
│   │ │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │ │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │ │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │ │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │ │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴─┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0006() {
  let plane = &mut Plane::new(EX_002);
  eq_cursor(1, 1, plane);
  plane.move_cursor(14, 4);
  plane.delete_character();
  eq_cursor(15, 5, plane);
  eq(EXPECTED_0006, plane);
}