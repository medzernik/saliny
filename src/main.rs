

struct Linka {
    cislo: uint,
    zastavky: Vec<Zastavka>
}

struct Odchod {
    cislo: uint,
    odchod: &str,
}

struct Zastavka {
    nazov: &str,
    harmonogram: Vec<Odchod>,
    pristup: bool,
}



fn main() {



}
