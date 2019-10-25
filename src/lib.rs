pub mod modulo00 {
    pub struct Dato(pub u32, pub u32);
    pub trait Consumible {
        fn consumir(&self, par01: &Dato) -> (u32, u32);
    }
    pub trait Bebible {
        fn beber(&self, par01: &Dato) -> (u32, u32);
    }
    pub struct Comensal;
    impl Comensal {
        pub fn almorzar(&self, par01: &Dato, par02: &Consumible, par03: &Bebible) -> (u32, u32) {
            let (a, b) = par02.consumir(par01);
            let (d, e) = par03.beber(par01);
            println!("{} {} {} {}", a, b, d, e);
            (a + b, d + e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modulo00::{Bebible, Consumible, Dato, Comensal};
    #[test]
    fn test_modulo00() {
        struct MiImpl {
            cadena: String,
        }

        impl Consumible for MiImpl {
            fn consumir(&self, par01: &Dato) -> (u32, u32) {
                println!("{}", self.cadena);
                (par01.0, par01.1)
            }
        }

        //la prueba unitaria
        let impl00 = MiImpl {
            cadena: String::from("hola gus"),
        };
        let (a, b) = impl00.consumir(&Dato(10, 10));
        println!("{},{}", a, b);
        assert_eq!(a, 10);
        assert_eq!(b, 10);
    }
    #[test]
    fn test_almuerzo() {
        struct MiImpl {
            cadena: String,
            valor: i32,
        }
        impl Consumible for MiImpl {
            fn consumir(&self, par01: &Dato) -> (u32, u32) {
                println!("{}", self.cadena);
                (par01.0, par01.1)
            }
        }
        impl Bebible for MiImpl {
            fn beber(&self, par01: &Dato) -> (u32, u32) {
                println!("{}", self.cadena.to_uppercase());
                (par01.0 + 1, par01.1 - 1)
            }
        }

        let mi_impl = MiImpl {
            cadena: String::from("ito"),
            valor: -15,
        };
        let martin = Comensal {};
        let (r1, r2) = martin.almorzar(&Dato(10, 10), &mi_impl, &mi_impl);
        println!("{} {}", r1, r2);
        assert_eq!(r1, 20);
        assert_eq!(r2, 20);
    }
}
