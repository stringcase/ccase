mod app;

pub use app::build as build_app;

use clap::ValueEnum;
use convert_case::{Case, Casing, Pattern};
use convert_case_extras::case::{ALTERNATING as ALTERNATING_CASE, PSEUDO_RANDOM as PSEUDO_RANDOM_CASE, RANDOM as RANDOM_CASE, TOGGLE as TOGGLE_CASE};
use convert_case_extras::pattern::{ALTERNATING, PSEUDO_RANDOM, RANDOM, TOGGLE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum CaseOption {
    Ada,
    Camel,
    Cobol,
    Constant,
    Flat,
    Kebab,
    Lower,
    Pascal,
    Sentence,
    Snake,
    Title,
    Train,
    Upper,
    UpperFlat,
    // from extras
    Alternating,
    PseudoRandom,
    Random,
    Toggle,
}

impl CaseOption {
    pub fn to_case(&self) -> Case<'static> {
        use convert_case::Case::*;
        match self {
            CaseOption::Ada => Ada,
            CaseOption::Camel => Camel,
            CaseOption::Cobol => Cobol,
            CaseOption::Constant => Constant,
            CaseOption::Flat => Flat,
            CaseOption::Kebab => Kebab,
            CaseOption::Lower => Lower,
            CaseOption::Pascal => Pascal,
            CaseOption::Sentence => Sentence,
            CaseOption::Snake => Snake,
            CaseOption::Title => Title,
            CaseOption::Train => Train,
            CaseOption::Upper => Upper,
            CaseOption::UpperFlat => UpperFlat,
            CaseOption::Alternating => ALTERNATING_CASE,
            CaseOption::PseudoRandom => PSEUDO_RANDOM_CASE,
            CaseOption::Random => RANDOM_CASE,
            CaseOption::Toggle => TOGGLE_CASE,
        }
    }

    pub fn name_in_case(&self) -> String {
        format!("{:?}Case", self).to_case(self.to_case())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum PatternOption {
    Lowercase,
    Uppercase,
    Capital,
    Sentence,
    Camel,
    Alternating,
    Toggle,
    PseudoRandom,
    Random,
}

impl PatternOption {
    pub fn to_pattern(&self) -> Pattern {
        use convert_case::Pattern::*;
        match self {
            PatternOption::Lowercase => Lowercase,
            PatternOption::Uppercase => Uppercase,
            PatternOption::Capital => Capital,
            PatternOption::Sentence => Sentence,
            PatternOption::Camel => Camel,
            PatternOption::Alternating => ALTERNATING,
            PatternOption::Toggle => TOGGLE,
            PatternOption::PseudoRandom => PSEUDO_RANDOM,
            PatternOption::Random => RANDOM,
        }
    }

    pub fn example(&self) -> &'static str {
        match self {
            PatternOption::Lowercase => "lower, lower, ...",
            PatternOption::Uppercase => "UPPER, UPPER, ...",
            PatternOption::Capital => "Capital, Capital, ...",
            PatternOption::Sentence => "Capital, lower, lower, ...",
            PatternOption::Camel => "lower, Capital, Capital, ...",
            PatternOption::Alternating => "aLtErNaTiNg, aLtErNaTiNg, ...",
            PatternOption::Toggle => "tOGGLE, tOGGLE, ...",
            PatternOption::PseudoRandom => "pSUeDorANdOm, pSUedORaNdoM, ...",
            PatternOption::Random => "RanDOM, RAndom, ...",
        }
    }
}

