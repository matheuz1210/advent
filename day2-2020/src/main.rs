pub fn main() {
    let count = include_str!("input")
        .lines()
        .filter(|i| Password::parse(i).filter(Password::is_valid).is_some())
        .count();

    println!("day2: {count} passwords were valid");
}

#[derive(Debug)]
struct PasswordPolicy {
    char: u8,
    range: [usize; 2],
}

impl PasswordPolicy {
    const fn is_valid(&self, pass: &str) -> bool {
        let pass = pass.as_bytes();
        (pass[self.range[0]] == self.char) ^ (pass[self.range[1]] == self.char)
    }
}

#[derive(Debug)]
struct Password<'a> {
    pass: &'a str,
    policy: PasswordPolicy,
}

impl<'a> Password<'a> {
    const fn is_valid(&self) -> bool {
        self.policy.is_valid(self.pass)
    }
    fn parse(i: &'a str) -> Option<Self> {
        let mut input = i.split(':');

        Some(Self {
            policy: {
                let mut parts = input.next()?.split(' ');
                PasswordPolicy {
                    range: {
                        let mut r = parts
                            .next()?
                            .split('-')
                            .filter_map(|i| i.parse::<usize>().ok());

                        [r.next()? - 1, r.next()? - 1]
                    },
                    char: parts.next()?.as_bytes()[0],
                }
            },
            pass: input.next()?.trim(),
        })
    }
}
