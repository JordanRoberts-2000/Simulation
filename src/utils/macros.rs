#[macro_export]
macro_rules! generate_getters {
  ($struct_name:ident, $( $method_name:ident : $field_type:ty ),* ) => {
      impl $struct_name {
          $(
              pub fn $method_name(&self) -> $field_type {
                  self.$method_name.clone()
              }
          )*
      }
  };
}

#[macro_export]
macro_rules! generate_mut_getters {
  ($struct_name:ident, $( $method_name:ident : $field_name:ident : $field_type:ty ),* ) => {
      impl $struct_name {
          $(
              pub fn $method_name(&mut self) -> &mut $field_type {
                  &mut self.$field_name
              }
          )*
      }
  };
}
