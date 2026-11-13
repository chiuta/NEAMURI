//! NEAMURI core: from a path through a family tree to the right kinship term.
//!
//! Seed rules for the common relatives; each language is a table in `term`.

/// One step along a family-tree path, seen from the starting person.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Parent,
    Child,
    Sibling,
    Spouse,
}

/// Sex of the person the term describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}

/// Language-independent kind of relative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kin {
    SelfPerson,
    Parent,
    Child,
    Sibling,
    Spouse,
    Grandparent,
    Grandchild,
    UncleAunt,
    NieceNephew,
    Cousin,
    ParentInLaw,
    ChildInLaw,
    SiblingInLaw,
}

/// Classify a path; `None` when the seed rules do not cover it (yet).
pub fn classify(path: &[Step]) -> Option<Kin> {
    use Step::*;
    Some(match path {
        [] => Kin::SelfPerson,
        [Parent] => Kin::Parent,
        [Child] => Kin::Child,
        [Sibling] => Kin::Sibling,
        [Spouse] => Kin::Spouse,
        [Parent, Parent] => Kin::Grandparent,
        [Child, Child] => Kin::Grandchild,
        [Parent, Sibling] => Kin::UncleAunt,
        [Sibling, Child] => Kin::NieceNephew,
        [Parent, Sibling, Child] => Kin::Cousin,
        [Spouse, Parent] => Kin::ParentInLaw,
        [Child, Spouse] => Kin::ChildInLaw,
        [Spouse, Sibling] | [Sibling, Spouse] => Kin::SiblingInLaw,
        _ => return None,
    })
}

/// The term for `kin` in `lang` (`"ro"`, `"en"`); `None` for an unknown language.
pub fn term(lang: &str, kin: Kin, sex: Sex) -> Option<&'static str> {
    let m = sex == Sex::Male;
    let pick = |male: &'static str, female: &'static str| if m { male } else { female };
    match lang {
        "ro" => Some(match kin {
            Kin::SelfPerson => "eu",
            Kin::Parent => pick("tată", "mamă"),
            Kin::Child => pick("fiu", "fiică"),
            Kin::Sibling => pick("frate", "soră"),
            Kin::Spouse => pick("soț", "soție"),
            Kin::Grandparent => pick("bunic", "bunică"),
            Kin::Grandchild | Kin::NieceNephew => pick("nepot", "nepoată"),
            Kin::UncleAunt => pick("unchi", "mătușă"),
            Kin::Cousin => pick("văr", "verișoară"),
            Kin::ParentInLaw => pick("socru", "soacră"),
            Kin::ChildInLaw => pick("ginere", "noră"),
            Kin::SiblingInLaw => pick("cumnat", "cumnată"),
        }),
        "en" => Some(match kin {
            Kin::SelfPerson => "self",
            Kin::Parent => pick("father", "mother"),
            Kin::Child => pick("son", "daughter"),
            Kin::Sibling => pick("brother", "sister"),
            Kin::Spouse => pick("husband", "wife"),
            Kin::Grandparent => pick("grandfather", "grandmother"),
            Kin::Grandchild => pick("grandson", "granddaughter"),
            Kin::UncleAunt => pick("uncle", "aunt"),
            Kin::NieceNephew => pick("nephew", "niece"),
            Kin::Cousin => "cousin",
            Kin::ParentInLaw => pick("father-in-law", "mother-in-law"),
            Kin::ChildInLaw => pick("son-in-law", "daughter-in-law"),
            Kin::SiblingInLaw => pick("brother-in-law", "sister-in-law"),
        }),
        _ => None,
    }
}

/// Parse a step name (`parent`, `child`, `sibling`, `spouse`).
pub fn parse_step(s: &str) -> Option<Step> {
    match s.to_ascii_lowercase().as_str() {
        "parent" => Some(Step::Parent),
        "child" => Some(Step::Child),
        "sibling" => Some(Step::Sibling),
        "spouse" => Some(Step::Spouse),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Step::*;

    #[test]
    fn romanian_terms() {
        let t = |p: &[Step], s| term("ro", classify(p).unwrap(), s).unwrap();
        assert_eq!(t(&[Parent, Parent], Sex::Female), "bunică");
        assert_eq!(t(&[Parent, Sibling], Sex::Male), "unchi");
        assert_eq!(t(&[Parent, Sibling], Sex::Female), "mătușă");
        assert_eq!(t(&[Parent, Sibling, Child], Sex::Female), "verișoară");
        assert_eq!(t(&[Spouse, Parent], Sex::Female), "soacră");
        assert_eq!(t(&[Sibling, Child], Sex::Male), "nepot");
    }

    #[test]
    fn english_terms_and_gaps() {
        assert_eq!(
            term("en", Kin::Grandchild, Sex::Female),
            Some("granddaughter")
        );
        assert_eq!(term("xx", Kin::Parent, Sex::Male), None);
        assert_eq!(classify(&[Parent, Child]), None);
        assert_eq!(parse_step("Spouse"), Some(Spouse));
    }
}
