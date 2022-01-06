

pub struct Linka {
    cislo: u64,
    zastavky: Vec<String>
}

impl Linka {
    pub fn create(cislo: u64, zastavky: Vec<String>) -> Linka {
        Linka {
            cislo: cislo,
            zastavky: zastavky,
        }

    }
}

struct Odchod {
    cislo: u64,
    odchod: String,
}

struct Zastavka {
    nazov: String,
    harmonogram: Vec<Odchod>,
    pristup: bool,
}



fn main() {
    let linka1 = Linka::create(2, vec![String::from("Hlavna stanica"), String::from("Pod stanicou")]);
    println!("Linka: {}, zastavky: {}", linka1.cislo, linka1.zastavky[0]);


}
