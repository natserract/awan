
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
pub struct OptChoice<Q> {
  pub default_value: Q,
}
pub trait Of<Q> {
  fn value(&self, arg: Option<Q>) -> Q;
}

impl<Q> Of<Q> for OptChoice<Q> 
where Q: Clone {
  fn value(&self, arg: Option<Q>) -> Q {
    let default_value = self.default_value.clone();

    match arg {
      Some (value) => value,
      None => default_value
    }
  }
}