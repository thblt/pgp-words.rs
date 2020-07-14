use err_derive::Error;
use std::env;
use std::io;
use std::process;

const WORDS: &'static [(&'static str, &'static str)] = &[
    ("aardvark", "adroitness"),
    ("absurd", "adviser"),
    ("accrue", "aftermath"),
    ("acme", "aggregate"),
    ("adrift", "alkali"),
    ("adult", "almighty"),
    ("afflict", "amulet"),
    ("ahead", "amusement"),
    ("aimless", "antenna"),
    ("Algol", "applicant"),
    ("allow", "Apollo"),
    ("alone", "armistice"),
    ("ammo", "article"),
    ("ancient", "asteroid"),
    ("apple", "Atlantic"),
    ("artist", "atmosphere"),
    ("assume", "autopsy"),
    ("Athens", "Babylon"),
    ("atlas", "backwater"),
    ("Aztec", "barbecue"),
    ("baboon", "belowground"),
    ("backfield", "bifocals"),
    ("backward", "bodyguard"),
    ("banjo", "bookseller"),
    ("beaming", "borderline"),
    ("bedlamp", "bottomless"),
    ("beehive", "Bradbury"),
    ("beeswax", "bravado"),
    ("befriend", "Brazilian"),
    ("Belfast", "breakaway"),
    ("berserk", "Burlington"),
    ("billiard", "businessman"),
    ("bison", "butterfat"),
    ("blackjack", "Camelot"),
    ("blockade", "candidate"),
    ("blowtorch", "cannonball"),
    ("bluebird", "Capricorn"),
    ("bombast", "caravan"),
    ("bookshelf", "caretaker"),
    ("brackish", "celebrate"),
    ("breadline", "cellulose"),
    ("breakup", "certify"),
    ("brickyard", "chambermaid"),
    ("briefcase", "Cherokee"),
    ("Burbank", "Chicago"),
    ("button", "clergyman"),
    ("buzzard", "coherence"),
    ("cement", "combustion"),
    ("chairlift", "commando"),
    ("chatter", "company"),
    ("checkup", "component"),
    ("chisel", "concurrent"),
    ("choking", "confidence"),
    ("chopper", "conformist"),
    ("Christmas", "congregate"),
    ("clamshell", "consensus"),
    ("classic", "consulting"),
    ("classroom", "corporate"),
    ("cleanup", "corrosion"),
    ("clockwork", "councilman"),
    ("cobra", "crossover"),
    ("commence", "crucifix"),
    ("concert", "cumbersome"),
    ("cowbell", "customer"),
    ("crackdown", "Dakota"),
    ("cranky", "decadence"),
    ("crowfoot", "December"),
    ("crucial", "decimal"),
    ("crumpled", "designing"),
    ("crusade", "detector"),
    ("cubic", "detergent"),
    ("dashboard", "determine"),
    ("deadbolt", "dictator"),
    ("deckhand", "dinosaur"),
    ("dogsled", "direction"),
    ("dragnet", "disable"),
    ("drainage", "disbelief"),
    ("dreadful", "disruptive"),
    ("drifter", "distortion"),
    ("dropper", "document"),
    ("drumbeat", "embezzle"),
    ("drunken", "enchanting"),
    ("Dupont", "enrollment"),
    ("dwelling", "enterprise"),
    ("eating", "equation"),
    ("edict", "equipment"),
    ("egghead", "escapade"),
    ("eightball", "Eskimo"),
    ("endorse", "everyday"),
    ("endow", "examine"),
    ("enlist", "existence"),
    ("erase", "exodus"),
    ("escape", "fascinate"),
    ("exceed", "filament"),
    ("eyeglass", "finicky"),
    ("eyetooth", "forever"),
    ("facial", "fortitude"),
    ("fallout", "frequency"),
    ("flagpole", "gadgetry"),
    ("flatfoot", "Galveston"),
    ("flytrap", "getaway"),
    ("fracture", "glossary"),
    ("framework", "gossamer"),
    ("freedom", "graduate"),
    ("frighten", "gravity"),
    ("gazelle", "guitarist"),
    ("Geiger", "hamburger"),
    ("glitter", "Hamilton"),
    ("glucose", "handiwork"),
    ("goggles", "hazardous"),
    ("goldfish", "headwaters"),
    ("gremlin", "hemisphere"),
    ("guidance", "hesitate"),
    ("hamlet", "hideaway"),
    ("highchair", "holiness"),
    ("hockey", "hurricane"),
    ("indoors", "hydraulic"),
    ("indulge", "impartial"),
    ("inverse", "impetus"),
    ("involve", "inception"),
    ("island", "indigo"),
    ("jawbone", "inertia"),
    ("keyboard", "infancy"),
    ("kickoff", "inferno"),
    ("kiwi", "informant"),
    ("klaxon", "insincere"),
    ("locale", "insurgent"),
    ("lockup", "integrate"),
    ("merit", "intention"),
    ("minnow", "inventive"),
    ("miser", "Istanbul"),
    ("Mohawk", "Jamaica"),
    ("mural", "Jupiter"),
    ("music", "leprosy"),
    ("necklace", "letterhead"),
    ("Neptune", "liberty"),
    ("newborn", "maritime"),
    ("nightbird", "matchmaker"),
    ("Oakland", "maverick"),
    ("obtuse", "Medusa"),
    ("offload", "megaton"),
    ("optic", "microscope"),
    ("orca", "microwave"),
    ("payday", "midsummer"),
    ("peachy", "millionaire"),
    ("pheasant", "miracle"),
    ("physique", "misnomer"),
    ("playhouse", "molasses"),
    ("Pluto", "molecule"),
    ("preclude", "Montana"),
    ("prefer", "monument"),
    ("preshrunk", "mosquito"),
    ("printer", "narrative"),
    ("prowler", "nebula"),
    ("pupil", "newsletter"),
    ("puppy", "Norwegian"),
    ("python", "October"),
    ("quadrant", "Ohio"),
    ("quiver", "onlooker"),
    ("quota", "opulent"),
    ("ragtime", "Orlando"),
    ("ratchet", "outfielder"),
    ("rebirth", "Pacific"),
    ("reform", "pandemic"),
    ("regain", "Pandora"),
    ("reindeer", "paperweight"),
    ("rematch", "paragon"),
    ("repay", "paragraph"),
    ("retouch", "paramount"),
    ("revenge", "passenger"),
    ("reward", "pedigree"),
    ("rhythm", "Pegasus"),
    ("ribcage", "penetrate"),
    ("ringbolt", "perceptive"),
    ("robust", "performance"),
    ("rocker", "pharmacy"),
    ("ruffled", "phonetic"),
    ("sailboat", "photograph"),
    ("sawdust", "pioneer"),
    ("scallion", "pocketful"),
    ("scenic", "politeness"),
    ("scorecard", "positive"),
    ("Scotland", "potato"),
    ("seabird", "processor"),
    ("select", "provincial"),
    ("sentence", "proximate"),
    ("shadow", "puberty"),
    ("shamrock", "publisher"),
    ("showgirl", "pyramid"),
    ("skullcap", "quantity"),
    ("skydive", "racketeer"),
    ("slingshot", "rebellion"),
    ("slowdown", "recipe"),
    ("snapline", "recover"),
    ("snapshot", "repellent"),
    ("snowcap", "replica"),
    ("snowslide", "reproduce"),
    ("solo", "resistor"),
    ("southward", "responsive"),
    ("soybean", "retraction"),
    ("spaniel", "retrieval"),
    ("spearhead", "retrospect"),
    ("spellbind", "revenue"),
    ("spheroid", "revival"),
    ("spigot", "revolver"),
    ("spindle", "sandalwood"),
    ("spyglass", "sardonic"),
    ("stagehand", "Saturday"),
    ("stagnate", "savagery"),
    ("stairway", "scavenger"),
    ("standard", "sensation"),
    ("stapler", "sociable"),
    ("steamship", "souvenir"),
    ("sterling", "specialist"),
    ("stockman", "speculate"),
    ("stopwatch", "stethoscope"),
    ("stormy", "stupendous"),
    ("sugar", "supportive"),
    ("surmount", "surrender"),
    ("suspense", "suspicious"),
    ("sweatband", "sympathy"),
    ("swelter", "tambourine"),
    ("tactics", "telephone"),
    ("talon", "therapist"),
    ("tapeworm", "tobacco"),
    ("tempest", "tolerance"),
    ("tiger", "tomorrow"),
    ("tissue", "torpedo"),
    ("tonic", "tradition"),
    ("topmost", "travesty"),
    ("tracker", "trombonist"),
    ("transit", "truncated"),
    ("trauma", "typewriter"),
    ("treadmill", "ultimate"),
    ("Trojan", "undaunted"),
    ("trouble", "underfoot"),
    ("tumor", "unicorn"),
    ("tunnel", "unify"),
    ("tycoon", "universe"),
    ("uncut", "unravel"),
    ("unearth", "upcoming"),
    ("unwind", "vacancy"),
    ("uproot", "vagabond"),
    ("upset", "vertigo"),
    ("upshot", "Virginia"),
    ("vapor", "visitor"),
    ("village", "vocalist"),
    ("virus", "voyager"),
    ("Vulcan", "warranty"),
    ("waffle", "Waterloo"),
    ("wallet", "whimsical"),
    ("watchword", "Wichita"),
    ("wayside", "Wilmington"),
    ("willow", "Wyoming"),
    ("woodlark", "yesteryear"),
    ("Zulu", "Yucatán"),
];

#[derive(Debug, Error)]
pub enum MyError {
    #[error(display = "Illegal character in input.")]
    InvalidCharacter,
    #[error(display = "Hex sequence length isn't a multiple of two.")]
    DanglingHalfByte,
}

/// Parse an hex string as a Vec of usize, removing all whitespace.
pub fn parse(l: &str) -> Result<Vec<usize>, MyError> {
    let mut ret: Vec<usize> = vec![];
    let mut between: bool = false;
    let mut msb: usize = 0;
    for c in l.to_lowercase().chars() {
        if c.is_ascii_hexdigit() {
            let digit: usize = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => panic!("Something went wrong (abnormal state)")
            };
            if between {
                ret.push(msb * 16 + digit);
                between = false;
            } else {
                msb = digit;
                between = true;
            }
        } else if !c.is_whitespace() {
            return Err(MyError::InvalidCharacter);
        }
    }
    if between
    {
        Err(MyError::DanglingHalfByte)
    } else {
        Ok(ret)
    }
}

/// Convert a hex string parsed by `parse()` to words.
pub fn convert (bytes: &Vec<usize>) -> Vec<&str> {
    let mut ret : Vec<&str> = vec!();
    let mut odd = true;
    for b in bytes {
        ret.push(if odd { WORDS[*b].0 } else { WORDS[*b].1 } );
        odd = !odd;
    }
    ret
}

/// For interactive use.  Convert an hex string to words, and display
/// the result.
pub fn convert_and_display(l: &str) {
    let bytes = parse(&l);

    match bytes {
        Ok(bytes) => {
            // We have bytes!
            let words = convert(&bytes);
            print!("{}:", l);
            for (i, w) in words.iter().enumerate() {
                if i % 4 == 0 { print!("\n\t") }
                print!("{} ", w);
            }
            println!("");
        }
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    if env::args().len() > 1 {
        // Work with command line
        let line = env::args().skip(1).collect::<String>();
        convert_and_display(&line);
    } else {
        // Work with stdin
        loop {
            let mut buf: String = String::new();
            if 0 == io::stdin().read_line(&mut buf).unwrap() {
                process::exit(0);
            }
        }
    }
}
