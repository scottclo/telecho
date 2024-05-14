use std::{io,env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target: TeleTarget = TeleTarget::boudot();
    for arg in &args[1..] {
        match arg.as_str() {
            "--boudot" => target = TeleTarget::boudot(),
            _ => {
                print!("Unknown argument: {}", arg);
                return;
            }
        }
    }
    if atty::is(atty::Stream::Stdin) {
        return();
    }

    let input: String = io::read_to_string(io::stdin()).unwrap();

    let output: String = input.chars().map(|c| {
        if c == '\n' || c == ' ' {
            c
        } else {
            target.filter(c)
        }
    }).collect();

    print!("{output}");
}

struct TeleTarget {
    uppercase: bool,
    lowercase: bool,
    char_set: Vec<char>,
    rules: Vec<ReplaceRule>,
}

impl TeleTarget {
    fn boudot() -> TeleTarget {
        TeleTarget{
            uppercase: true,
            lowercase: false,
            char_set: vec![
                'Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M',
                '1','2','3','4','5','6','7','8','9','0','-','$','!','&','#','\'','(',')','"','/',':',':',';','?',',','.'
            ],
            rules: vec![
                ReplaceRule{
                    targets: vec!['}','\\'],
                    replace: ')',
                },
                ReplaceRule{
                    targets: vec!['{'],
                    replace: '(',
                },
                ReplaceRule{
                    targets: vec!['@', '+'],
                    replace: '#',
                },
                ReplaceRule{
                    targets: vec!['`'],
                    replace: '\'',
                },
                ReplaceRule{
                    targets: vec!['_'],
                    replace: '.',
                },
                ReplaceRule{
                    targets: vec!['<','>','~'],
                    replace: '-',
                },
                ReplaceRule{
                    targets: vec!['*'],
                    replace: '"',
                },
                ReplaceRule{
                    targets: vec!['|'],
                    replace: '!',
                }
            ]
        }
    }
    fn filter(&self, mut c: char) -> char{
        if self.uppercase && !self.lowercase{
            c = c.to_ascii_uppercase();
        } else if self.lowercase && !self.uppercase{
            c = c.to_ascii_lowercase();
        } 
        if !self.char_set.iter().any(|&x| x == c) {
            for rule in self.rules.iter() {
                match rule.replace(c) {
                    Some(x) => return x,
                    None => (),
                }
            }
            return ' ';
        }
        return c;
    }
}

#[derive(Debug)]
struct ReplaceRule {
    targets: Vec<char>,
    replace: char
}

impl ReplaceRule {
    fn replace (&self, c: char) -> Option<char> {
        if self.targets.iter().any(|&x| x == c) {
            Some(self.replace)
        }else {
            None
        }
    }
}
