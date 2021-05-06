pub trait Rule {
    type ErrType;

    fn id(&self) -> String;
    fn check(&self) -> Result<(), Self::ErrType>;
    fn fix(&self) {}
}

pub struct RuleConf {
    pub wd: String,
}

pub struct K000220<'a>(pub &'a RuleConf);
pub struct K002620<'a>(pub &'a RuleConf);

pub mod k000220;
pub mod k002620;
