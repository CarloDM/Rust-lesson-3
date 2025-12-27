pub mod modulo_test ;
// use crate::modulo_test::modulo_test2::Prova ;
pub use crate::modulo_test::Prova ;

fn main() {
    let test_a = Prova{
      uno : 1,
      due : 2,
    };

  println!("fine prog  {:?}", test_a );
}
