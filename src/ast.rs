#[derive(Debug, Clone)]
pub struct Grammar {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    name: String,
}

#[derive(Debug, Clone)]
pub enum RuleKind {
    Enum(Vec<(RuleBody, String)>),
    Normal(RuleBody),
    Empty,
}

#[derive(Debug, Clone)]
pub struct RuleBody {
    body: Vec<RuleElement>,
}

#[derive(Debug, Clone)]
pub struct RuleElement {
    name: Option<String>,
    nt: String,
    quantifier: Option<Quantifier>,
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
