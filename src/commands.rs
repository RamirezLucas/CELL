use std::collections::HashMap;

pub struct Command {
    keyword: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(keyword: &str, args: Vec<&str>) -> Self {
        Self {
            keyword: String::from(keyword),
            args: args.into_iter().map(|x| String::from(x)).collect(),
        }
    }

    pub fn match_cmd<'a, 'b>(&'a self, cmd: &'b str) -> Option<HashMap<&'a str, &'b str>> {
        let tokens: Vec<&str> = cmd.split_whitespace().collect();
        {
            if tokens.len() == (1 + self.args.len()) && tokens[0] == self.keyword {
                let mut map = HashMap::new();
                for token_arg in self.args.iter().zip(tokens[1..].iter()) {
                    let (argname, token) = token_arg;
                    map.insert(&argname[..], *token);
                }
                Some(map)
            } else {
                None
            }
        }
    }

    pub fn get_keyword(&self) -> &str {
        &self.keyword[..]
    }
}
