// # The `OptChoice` Struct
//
// -> Struct for represents value with two possibilities
// -> Value of type: Some will return `Arg`, None => `default value`
//
// # Examples
// ```
//  let catch = Some(String::from("world"));
//
//  let example = OptChoice {
//    default_value: "hello".to_string()
//  };
//
//  assert_eq!(example.value(catch), "world");
// ```
//
pub struct OptChoice<T> {
  pub default_value: T,
}
pub trait Of<T> {
  fn value(&self, arg: Option<T>) -> T;
}

impl<T> Of<T> for OptChoice<T>
where
  T: Clone,
{
  fn value(&self, arg: Option<T>) -> T {
    let default_value = self.default_value.clone();

    match arg {
      Some(value) => value,
      None => default_value,
    }
  }
}
