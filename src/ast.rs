use reacto::ast::N;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: Vec<N<Rule>>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub attrs: Vec<N<Attr>>,
    pub name: N<Ident>,
    pub kind: RuleKind,
}

#[derive(Debug, Clone)]
pub enum RuleKind {
    Enum(Vec<N<NamedRuleBody>>),
    Normal(N<RuleBody>),
}

#[derive(Debug, Clone)]
pub struct NamedRuleBody {
    pub name: N<Ident>,
    pub body: Option<N<RuleBody>>,
}

#[derive(Debug, Clone)]
pub struct RuleBody {
    pub body: Vec<N<RuleElement>>,
}

#[derive(Debug, Clone)]
pub struct RuleElement {
    pub name: Option<N<Ident>>,
    pub nt: N<Ident>,
    pub quantifier: Option<N<Quantifier>>,
}

impl RuleElement {
    pub fn has_many(&self) -> bool {
        if let Some(d) = &self.quantifier {
            if d.data == Quantifier::Multi || d.data == Quantifier::AtLeastOne {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Quantifier {
    /// ?
    Maybe,
    /// *
    Multi,
    /// +
    AtLeastOne,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Attr {
    pub name: String,
}

impl Quantifier {
    pub fn to_str(&self) -> &str {
        use Quantifier::*;
        match self {
            Maybe => "?",
            Multi => "*",
            AtLeastOne => "+",
        }
    }
}

impl Ident {
    pub fn to_str(&self) -> &str {
        &self.name
    }
}

impl Attr {
    pub fn to_str(&self) -> &str {
        &self.name
    }
}
