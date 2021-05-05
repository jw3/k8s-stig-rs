pub trait Rule {
    type ErrType;

    fn id(&self) -> String;
    fn check(&self) -> Result<(), Self::ErrType>;
    fn fix(&self) {}
}

pub struct RuleConf {
    pub wd: String,
}

pub struct K220<'a>(pub &'a RuleConf);
pub struct K2620<'a>(pub &'a RuleConf);

pub mod k220;
pub mod k2620;
