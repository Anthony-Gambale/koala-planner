
use super::*;

#[cfg(test)]
#[test]
pub fn my_first_test() {
  let x: i32 = my_first_function(None, None);
  assert_eq!(x, 2);
}
