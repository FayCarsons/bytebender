use crate::utils::Weights;

pub const HEADER_LEN: u32 = 1024u32 * 16u32;
pub const FOOTER_LEN: u16 = 0u16;
pub const CHUNK_SIZE: usize = 1024 * 16;
pub const NUM_CHUNKS: usize = 2048;

pub const INPUT_PATH: &str = "resources/kigg.jpg";
pub const OUTPUT_PATH: &str = "test8.jpg";

pub const WEIGHTS: Weights = Weights {
    normal: 1000,
    shuffle: 0,
    sorted: 100,
    repeat: 1000,
    replace: 1000,
    to_string: 100,
    noise: 0,
    nop: 0,
};

pub const PHRASES: [&str; 8] = [
    "Hello, world",
    "I Love You",
    "I am a fish",
    "You are a fish",
    "we aer all fish",
    "And I love you a lot, because you are a fish",
    "apfhpah398ha9thpaqh89vm9n8ayna2ynt9ya2n39t8yna",
    "Groupers are fish of any of a number of genera in the subfamily Epinephelinae of the family Serranidae, in the order Perciformes.

    Not all serranids are called \"groupers\"; the family also 
    includes the sea basses. The common name \"grouper\" is 
    usually given to fish in one of two large genera: 
    Epinephelus and Mycteroperca. In addition, the species 
    classified in the small genera Anyperidon, Cromileptes, 
    Dermatolepis, Graciela, Saloptia, and Triso are also called 
\"groupers.\" Fish in the genus Plectropomus are referred to as 
    \"coral groupers.\" These genera are all classified in the subfamily Epiphelinae. 
    However, some of the hamlets (genus Alphestes), the hinds (genus Cephalopholis), 
    the lyretails (genus Variola), and some other small genera (Gonioplectrus, Niphon, Paranthias) 
    are also in this subfamily, and occasional species in other serranid genera have common names 
    involving the word \"grouper.\" Nonetheless, the word \"grouper\" on 
    its own is usually taken as meaning the subfamily Epinephelinae."
];
