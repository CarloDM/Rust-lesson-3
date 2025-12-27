mod modulo_test ;
// use crate::modulo_test::modulo_test2::Prova ;
use crate::modulo_test::Prova ;

fn main() {
    let test_a = Prova{
      uno : 1,
      due : 2,
    };
    
  print!("{} , {} ", test_a.uno, test_a.due);
  println!("fine prog  {:?}", test_a );
}
