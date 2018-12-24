use super::*;

#[test]
fn it_works() {
  let mut input = Vec::new();
  input.push("+1".to_string());
  assert_eq!(1, frequency(0,input));
}

#[test]
fn it_works2() {
  let mut input = Vec::new();
  input.push("-2".to_string());
  assert_eq!(-1, frequency(1,input));
}

#[test]
fn it_works3() {
  let mut input = Vec::new();
  input.push("+3".to_string());
  assert_eq!(2, frequency(-1,input));
}

#[test]
fn it_works4() {
  let mut input = Vec::new();
  input.push("+1".to_string());
  assert_eq!(3, frequency(2,input));
}

#[test]
fn it_works5() {
  let mut input = Vec::new();
  input.push("+2".to_string());
  input.push("+4".to_string());
  input.push("-4".to_string());
  assert_eq!(3, frequency(1,input));
}