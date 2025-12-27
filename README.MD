### Packages, Crates, and Modules

pub mod → rende visibile il modulo dall’esterno (main.rs in questo caso)

use crate::... → crea una scorciatoia per arrivare a quello che ti serve

Sottomoduli → puoi organizzare tutto il codice in pezzi logici senza confonderti

✅ Risultato: codice pulito, modulare, riutilizzabile.

```
pub mod modulo_test
use crate::modulo_test:modulo_test2:Prova
```


### accorciare e semplificare i percorsi dei sottomoduli

```
// modulo_test.rs
pub use modulo_test2::Prova;

// main.rs
pub use crate::modulo_test::Prova ;
// anziche
use crate::modulo_test:modulo_test2:Prova ;

```


Perché è utile

Riduce lunghezza dei percorsi

Mantiene incapsulamento e visibilità

Consente di cambiare la struttura interna senza rompere chi usa il modulo

Ideale per librerie e workspace più grandi