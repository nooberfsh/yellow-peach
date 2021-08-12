#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub kind: RuleKind,
}

#[derive(Debug, Clone)]
pub enum RuleKind {
    Enum(Vec<(RuleBody, String)>),
    Normal(RuleBody),
    Empty,
}

#[derive(Debug, Clone)]
pub struct RuleBody {
    pub body: Vec<RuleElement>,
}

#[derive(Debug, Clone)]
pub struct RuleElement {
    pub name: Option<String>,
    pub nt: String,
    pub quantifier: Option<Quantifier>,
}

#[derive(Debug, Clone)]
pub enum  Quantifier{
    /// ?
    Maybe,
    /// *
    Multi,
    /// +
    AtLeastOne,
}
