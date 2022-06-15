// build.rs

use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A unicode general category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    // Leters
    Lu,
    Ll,
    Lt,
    Lm,
    Lo,

    // Mark
    Mn,
    Mc,
    Me,

    // Number
    Nd,
    Nl,
    No,

    // Punctuation
    Pc,
    Pd,
    Ps,
    Pe,
    Pi,
    Pf,
    Po,

    // Symbol
    Sm,
    Sc,
    Sk,
    So,

    // Seperator
    Zs,
    Zl,
    Zp,

    // Other
    Cc,
    Cf,
    Cs,
    Co,
    Cn,
}

impl Category {
    fn from_category_str<S: AsRef<str>>(category: S) -> Option<Self> {
        match category.as_ref() {
            "Lu" => Some(Self::Lu),
            "Ll" => Some(Self::Ll),
            "Lt" => Some(Self::Lt),
            "Lm" => Some(Self::Lm),
            "Lo" => Some(Self::Lo),
            "Mn" => Some(Self::Mn),
            "Mc" => Some(Self::Mc),
            "Me" => Some(Self::Me),
            "Nd" => Some(Self::Nd),
            "Nl" => Some(Self::Nl),
            "No" => Some(Self::No),
            "Pc" => Some(Self::Pc),
            "Pd" => Some(Self::Pd),
            "Ps" => Some(Self::Ps),
            "Pe" => Some(Self::Pe),
            "Pi" => Some(Self::Pi),
            "Pf" => Some(Self::Pf),
            "Po" => Some(Self::Po),
            "Sm" => Some(Self::Sm),
            "Sc" => Some(Self::Sc),
            "Sk" => Some(Self::Sk),
            "So" => Some(Self::So),
            "Zs" => Some(Self::Zs),
            "Zl" => Some(Self::Zl),
            "Zp" => Some(Self::Zp),
            "Cc" => Some(Self::Cc),
            "Cf" => Some(Self::Cf),
            "Cs" => Some(Self::Cs),
            "Co" => Some(Self::Co),
            "Cn" => Some(Self::Cn),
            _ => None,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Lu => "Lu",
            Self::Ll => "Ll",
            Self::Lt => "Lt",
            Self::Lm => "Lm",
            Self::Lo => "Lo",
            Self::Mn => "Mn",
            Self::Mc => "Mc",
            Self::Me => "Me",
            Self::Nd => "Nd",
            Self::Nl => "Nl",
            Self::No => "No",
            Self::Pc => "Pc",
            Self::Pd => "Pd",
            Self::Ps => "Ps",
            Self::Pe => "Pe",
            Self::Pi => "Pi",
            Self::Pf => "Pf",
            Self::Po => "Po",
            Self::Sm => "Sm",
            Self::Sc => "Sc",
            Self::Sk => "Sk",
            Self::So => "So",
            Self::Zs => "Zs",
            Self::Zl => "Zl",
            Self::Zp => "Zp",
            Self::Cc => "Cc",
            Self::Cf => "Cf",
            Self::Cs => "Cs",
            Self::Co => "Co",
            Self::Cn => "Cn",
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct UnicodeMapping {
    character: u32,
    category: Category,
}

impl UnicodeMapping {
    fn new(character: u32, category: Category) -> Self {
        Self {
            character,
            category,
        }
    }
}

fn generate_char_to_mapping<P: AsRef<Path>>(
    mapping_filename: P,
) -> Result<Vec<UnicodeMapping>, String> {
    let file = File::open(mapping_filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.map_err(|err| format!("{:?}", err)).and_then(|line| {
                let mut columns = line.splitn(4, ';').take(3);
                let ch_col = columns
                    .next()
                    .ok_or_else(|| "missing character column".to_string())?;

                let ch = u32::from_str_radix(ch_col, 16)
                    .ok()
                    .ok_or_else(|| format!("cannot convert (0x{}) to char", ch_col))?;

                // skip the second column
                let category = columns.nth(1).unwrap();
                let category = Category::from_category_str(category).ok_or_else(|| {
                    format!("cannot generate Category from invalid code ({})", category)
                })?;

                Ok(UnicodeMapping::new(ch, category))
            })
        })
        // filter non-general mappings
        .filter(|line| line.is_ok())
        .collect::<Result<Vec<UnicodeMapping>, _>>()
}

fn generate_char_to_category(mappings: &[UnicodeMapping]) -> String {
    let preamble = "fn unicode_category_str_from_char(c: char) -> Option<&'static str> {
    let c  = c as u32;
    match c {\n";
    let postamble = "       _ => None,    
    }
}";

    let mut peekable_mappings = mappings.iter().peekable();
    let mut sos: u32 = peekable_mappings
        .peek()
        .map(|mapping| mapping.character)
        .unwrap_or(0);

    let mut sequences = vec![];

    loop {
        let mapping = peekable_mappings.next();
        let next = peekable_mappings.peek();

        match (mapping, next) {
            (None, _) => break,
            // increment eos if the mapping is a sequence matches
            (Some(mapping), Some(next))
                if next.character == (mapping.character + 1)
                    && next.category == mapping.category =>
            {
                continue;
            }
            (Some(mapping), Some(next)) => {
                let start = sos;
                let end = mapping.character;
                let category = mapping.category;

                // set the next sequence elements bounds
                sos = next.character;

                sequences.push((start, end, category))
            }
            (Some(mapping), None) => {
                let start = sos;
                let end = mapping.character;
                let category = mapping.category;

                sequences.push((start, end, category))
            }
        }
    }

    let mappings = sequences.iter().map(|(start, end, category)| {
        format!(
            "       {}..={} => Some(\"{}\"),\n",
            start,
            end,
            category.as_str()
        )
    });

    vec![preamble.to_string()]
        .into_iter()
        .chain(mappings)
        .chain(vec![postamble.to_string()].into_iter())
        .collect()
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("unicode_mappings.rs");
    let cur_dir = env::current_dir().unwrap();
    let mapping_path = Path::new(&cur_dir).join("assets").join("UnicodeData.txt");
    let mappings = generate_char_to_mapping(&mapping_path).unwrap();
    let char_to_category_str = generate_char_to_category(&mappings);

    fs::write(&dest_path, char_to_category_str).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
