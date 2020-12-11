use regex::Regex;
use ron::to_string;
use std::ops::Range;
use std::rc::Rc;

pub enum ParserKind {
    Regex(Regex),
    And,
    Ignore,
    Or,
    RepeatRange(Range<usize>),
    Map(Rc<Box<dyn Fn(String) -> Result<String, ron::Error>>>),
    Custom(Rc<Box<dyn Fn(String) -> Result<(String, String), String>>>),
}
impl std::fmt::Debug for ParserKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::fmt::Display for ParserKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserKind::*;
        match self {
            Regex(r) => write!(f, "Regex /{}/", r.as_str()),
            And => write!(f, "And"),
            Ignore => write!(f, "Ignore"),
            Or => write!(f, "Or"),
            RepeatRange(range) => write!(f, "RepeatRange {:?}", range),
            Map(_) => write!(f, "Map"),
            Custom(_) => write!(f, "Custom"),
        }
    }
}
impl Clone for ParserKind {
    fn clone(&self) -> Self {
        use ParserKind::*;
        match self {
            Regex(r) => Regex(r.clone()),
            And => And,
            Ignore => Ignore,
            Or => Or,
            RepeatRange(range) => RepeatRange(range.clone()),
            Map(cfn) => Map(Rc::clone(cfn)),
            Custom(cfn) => Custom(Rc::clone(cfn)),
        }
    }
}
impl PartialEq for ParserKind {
    fn eq(&self, other: &ParserKind) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    kind: ParserKind,
    subparsers: Vec<Parser>,
}
impl std::fmt::Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}
impl Parser {
    pub fn parse<T: Into<String>>(&self, src: T) -> Result<(String, String), String> {
        use ParserKind::*;
        let s: String = src.into();
        match &self.kind {
            Regex(re) => {
                if let Some(mat) = re.find(&s) {
                    if mat.start() == 0 {
                        Ok((
                            s[mat.start()..mat.end()].to_owned(),
                            s[mat.end()..].to_owned(),
                        ))
                    } else {
                        Err(s)
                    }
                } else {
                    Err(s)
                }
            }
            And => {
                if self.subparsers[0].kind == Ignore && self.subparsers[1].kind == Ignore {
                    Ok(("".into(), s))
                } else if self.subparsers[0].kind == Ignore {
                    let (_, rest) = self.subparsers[0].parse(s)?;
                    self.subparsers[1].parse(rest)
                } else if self.subparsers[1].kind == Ignore {
                    let (matched, lrest) = self.subparsers[0].parse(s.clone())?;
                    if let Ok((_, rest)) = self.subparsers[1].parse(lrest) {
                        Ok((matched, rest))
                    } else {
                        Err(s)
                    }
                } else {
                    let (lmatched, lrest) = self.subparsers[0].parse(s)?;
                    let (rmatched, rrest) = self.subparsers[1].parse(lrest)?;
                    Ok((
                        to_string(&vec![lmatched.clone(), rmatched.clone()]).unwrap(),
                        rrest,
                    ))
                }
            }
            Ignore => Ok(("".into(), self.subparsers[0].parse(s)?.1)),
            Or => {
                if let Ok(lresult) = self.subparsers[0].parse(s.clone()) {
                    Ok(lresult)
                } else {
                    self.subparsers[1].parse(s.clone())
                }
            }
            RepeatRange(range) => {
                let mut matched = vec![];
                let mut rest = s.clone();

                // Parse up to range.start
                for _ in 0..range.start {
                    let (m, r) = self.subparsers[0].parse(rest)?;
                    matched.push(m);
                    rest = r;
                }

                // Parse optionally up to range.end
                for _ in 0..(range.end - range.start) {
                    let parse_result = self.subparsers[0].parse(rest);
                    if let Err(r) = parse_result {
                        rest = r;
                        break;
                    } else {
                        let (m, r) = parse_result.unwrap();
                        matched.push(m);
                        rest = r;
                    }
                }

                Ok((to_string(&matched).unwrap(), rest))
            }
            Map(cfn) => {
                let (matched, rest) = self.subparsers[0].parse(s)?;
                if let Ok(m) = cfn(matched) {
                    Ok((m, rest))
                } else {
                    Err(rest)
                }
            }
            Custom(cfn) => cfn(s),
        }
    }

    // Static
    pub fn regex<T: Into<String>>(s: T) -> Parser {
        Parser {
            kind: ParserKind::Regex(Regex::new(&s.into()).expect("could not compile regex")),
            subparsers: vec![],
        }
    }
    pub fn custom<F: 'static>(cfn: F) -> Parser
    where
        F: Fn(String) -> Result<(String, String), String>,
    {
        Parser {
            kind: ParserKind::Custom(Rc::new(Box::new(cfn))),
            subparsers: vec![],
        }
    }

    // Instance
    pub fn and(self, r: Parser) -> Parser {
        Parser {
            kind: ParserKind::And,
            subparsers: vec![self, r],
        }
    }
    pub fn ignore(self) -> Parser {
        Parser {
            kind: ParserKind::Ignore,
            subparsers: vec![self],
        }
    }
    pub fn or(self, r: Parser) -> Parser {
        Parser {
            kind: ParserKind::Or,
            subparsers: vec![self, r],
        }
    }
    pub fn repeat_range(self, num_repeats: Range<usize>) -> Parser {
        Parser {
            kind: ParserKind::RepeatRange(num_repeats),
            subparsers: vec![self],
        }
    }
    pub fn optional(self) -> Parser {
        Parser {
            kind: ParserKind::RepeatRange(0..1),
            subparsers: vec![self],
        }
    }
    pub fn map<F: 'static>(self, cfn: F) -> Parser
    where
        F: Fn(String) -> Result<String, ron::Error>,
    {
        Parser {
            kind: ParserKind::Map(Rc::new(Box::new(cfn))),
            subparsers: vec![self],
        }
    }

    // Other
    pub fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        for _ in 0..indent {
            write!(f, " ")?;
        }
        write!(f, "{}", self.kind)?;
        if self.subparsers.len() > 0 {
            write!(f, " [\n")?;
            for subparser in &self.subparsers {
                subparser.pretty_print(f, indent + 2)?;
                write!(f, ",\n")?;
            }
            for _ in 0..indent {
                write!(f, " ")?;
            }
            write!(f, "]")
        } else {
            write!(f, "")
        }
    }
}
