fn main() {
    let g2p = g2p::G2p::new();
    let texts = vec![
        "I have $250 in my pocket.", // number -> spell-out
        "popular pets, e.g. cats and dogs", // e.g. -> for example
        "I refuse to collect the refuse around here.", // homograph
        "I'm an activationist.", // newly coined word
    ];
    for text in texts {
        let out = g2p.convert(text);
        println!("{}", out);
    }
}