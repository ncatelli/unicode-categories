include!(concat!(env!("OUT_DIR"), "/unicode_mappings.rs"));

/// Provides extention methods to allow `char` to optionally return it's
/// associated unicode category.
pub trait UnicodeCategorizable {
    fn unicode_category(&self) -> Option<Category>;
}

impl UnicodeCategorizable for char {
    /// Returns a `char`'s corresponding general unicode category or returns none.
    ///
    /// # Example
    ///
    /// ```
    /// use unicode_categories::*;
    ///
    /// assert_eq!(Some(Category::Lu), 'A'.unicode_category());
    /// assert_eq!(Some(Category::Ll), 'a'.unicode_category());
    /// ```
    fn unicode_category(&self) -> Option<Category> {
        unicode_category_from_char(*self)
    }
}

/// A unicode general category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    // Leters
    Lu,
    Ll,
    Lt,
    Lm,
    Lo,

    // Mark
    Mn,
    Mc,
    Me,

    // Punctuation
    Pc,
    Pd,
    Ps,
    Pe,
    Pi,
    Pf,
    Po,

    // Symbol
    Sm,
    Sc,
    Sk,
    So,

    // Seperator
    Zs,
    Zl,
    Zp,

    // Other
    Cc,
    Cf,
    Cs,
    Co,
    Cn,
}

impl Category {
    pub fn from_category_str<S: AsRef<str>>(category: S) -> Option<Self> {
        match category.as_ref() {
            "Lu" => Some(Self::Lu),
            "Ll" => Some(Self::Ll),
            "Lt" => Some(Self::Lt),
            "Lm" => Some(Self::Lm),
            "Lo" => Some(Self::Lo),
            "Mn" => Some(Self::Mn),
            "Mc" => Some(Self::Mc),
            "Me" => Some(Self::Me),
            "Pc" => Some(Self::Pc),
            "Pd" => Some(Self::Pd),
            "Ps" => Some(Self::Ps),
            "Pe" => Some(Self::Pe),
            "Pi" => Some(Self::Pi),
            "Pf" => Some(Self::Pf),
            "Po" => Some(Self::Po),
            "Sm" => Some(Self::Sm),
            "Sc" => Some(Self::Sc),
            "Sk" => Some(Self::Sk),
            "So" => Some(Self::So),
            "Zs" => Some(Self::Zs),
            "Zl" => Some(Self::Zl),
            "Zp" => Some(Self::Zp),
            "Cc" => Some(Self::Cc),
            "Cf" => Some(Self::Cf),
            "Cs" => Some(Self::Cs),
            "Co" => Some(Self::Co),
            "Cn" => Some(Self::Cn),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Lu => "Lu",
            Self::Ll => "Ll",
            Self::Lt => "Lt",
            Self::Lm => "Lm",
            Self::Lo => "Lo",
            Self::Mn => "Mn",
            Self::Mc => "Mc",
            Self::Me => "Me",
            Self::Pc => "Pc",
            Self::Pd => "Pd",
            Self::Ps => "Ps",
            Self::Pe => "Pe",
            Self::Pi => "Pi",
            Self::Pf => "Pf",
            Self::Po => "Po",
            Self::Sm => "Sm",
            Self::Sc => "Sc",
            Self::Sk => "Sk",
            Self::So => "So",
            Self::Zs => "Zs",
            Self::Zl => "Zl",
            Self::Zp => "Zp",
            Self::Cc => "Cc",
            Self::Cf => "Cf",
            Self::Cs => "Cs",
            Self::Co => "Co",
            Self::Cn => "Cn",
        }
    }
}

impl From<HumanReadableCategory> for Category {
    fn from(hrc: HumanReadableCategory) -> Self {
        match hrc {
            HumanReadableCategory::LetterUppercase => Self::Cc,
            HumanReadableCategory::LetterLowercase => Self::Ll,
            HumanReadableCategory::LetterTitlecase => Self::Lt,
            HumanReadableCategory::LetterModifier => Self::Lm,
            HumanReadableCategory::LetterOther => Self::Lo,
            HumanReadableCategory::MarkNonspacing => Self::Mn,
            HumanReadableCategory::MarkSpacingCombining => Self::Mc,
            HumanReadableCategory::MarkEnclosing => Self::Me,
            HumanReadableCategory::PunctuationConnector => Self::Pc,
            HumanReadableCategory::PunctuationDash => Self::Pd,
            HumanReadableCategory::PunctuationOpen => Self::Ps,
            HumanReadableCategory::PunctuationClose => Self::Pe,
            HumanReadableCategory::PunctuationInnerQuote => Self::Pi,
            HumanReadableCategory::PunctuationFinalQuote => Self::Pf,
            HumanReadableCategory::PunctuationOther => Self::Po,
            HumanReadableCategory::SymbolMath => Self::Sm,
            HumanReadableCategory::SymbolCurrency => Self::Sc,
            HumanReadableCategory::SymbolModifier => Self::Sk,
            HumanReadableCategory::SymbolOther => Self::So,
            HumanReadableCategory::SeperatorSpace => Self::Zs,
            HumanReadableCategory::SeperatorLine => Self::Zl,
            HumanReadableCategory::SeperatorParagraph => Self::Zp,
            HumanReadableCategory::OtherControl => Self::Cc,
            HumanReadableCategory::OtherFormat => Self::Cf,
            HumanReadableCategory::OtherSurrogate => Self::Cs,
            HumanReadableCategory::OtherPrivateUse => Self::Co,
            HumanReadableCategory::OtherNotAssigned => Self::Cn,
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A unicode general category in a more human-readable format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanReadableCategory {
    // Leters
    /// `Lu`
    LetterUppercase,
    /// `Ll`
    LetterLowercase,
    /// `Lt`
    LetterTitlecase,
    /// `Lm`
    LetterModifier,
    /// `Lo`
    LetterOther,

    // Mark
    /// `Mn`
    MarkNonspacing,
    /// `Mc`
    MarkSpacingCombining,
    /// `Me`
    MarkEnclosing,

    // Punctuation
    /// `Pc`
    PunctuationConnector,
    /// `Pd`
    PunctuationDash,
    /// `Ps`
    PunctuationOpen,
    /// `Pe`
    PunctuationClose,
    /// `Pi`
    PunctuationInnerQuote,
    /// `Pf`
    PunctuationFinalQuote,
    /// `Po`
    PunctuationOther,

    // Symbol
    /// `Sm`
    SymbolMath,
    /// `Sc`
    SymbolCurrency,
    /// `Sk`
    SymbolModifier,
    /// `So`
    SymbolOther,

    // Seperator
    /// `Zs`
    SeperatorSpace,
    /// `Zl`
    SeperatorLine,
    /// `Zp`
    SeperatorParagraph,

    // Other
    /// `Cc`
    OtherControl,
    /// `Cf`
    OtherFormat,
    /// `Cs`
    OtherSurrogate,
    /// `Co`
    OtherPrivateUse,
    /// `Cn`
    OtherNotAssigned,
}

impl From<Category> for HumanReadableCategory {
    fn from(cat: Category) -> Self {
        match cat {
            Category::Lu => Self::LetterUppercase,
            Category::Ll => Self::LetterLowercase,
            Category::Lt => Self::LetterTitlecase,
            Category::Lm => Self::LetterModifier,
            Category::Lo => Self::LetterOther,
            Category::Mn => Self::MarkNonspacing,
            Category::Mc => Self::MarkSpacingCombining,
            Category::Me => Self::MarkEnclosing,
            Category::Pc => Self::PunctuationConnector,
            Category::Pd => Self::PunctuationDash,
            Category::Ps => Self::PunctuationOpen,
            Category::Pe => Self::PunctuationClose,
            Category::Pi => Self::PunctuationInnerQuote,
            Category::Pf => Self::PunctuationFinalQuote,
            Category::Po => Self::PunctuationOther,
            Category::Sm => Self::SymbolMath,
            Category::Sc => Self::SymbolCurrency,
            Category::Sk => Self::SymbolModifier,
            Category::So => Self::SymbolOther,
            Category::Zs => Self::SeperatorSpace,
            Category::Zl => Self::SeperatorLine,
            Category::Zp => Self::SeperatorParagraph,
            Category::Cc => Self::OtherControl,
            Category::Cf => Self::OtherFormat,
            Category::Cs => Self::OtherSurrogate,
            Category::Co => Self::OtherPrivateUse,
            Category::Cn => Self::OtherNotAssigned,
        }
    }
}

/// Generates a corresponding general unicode category for a provide char. If
/// the character isn't a member of a general category, `Option::None` is
/// returned.
///
/// # Example
///
/// ```
/// use unicode_categories::*;
///
/// assert_eq!(Some(Category::Lu), unicode_category_from_char('A'));
/// assert_eq!(Some(Category::Ll), unicode_category_from_char('a'));
/// ```
pub fn unicode_category_from_char(c: char) -> Option<Category> {
    unicode_category_str_from_char(c).and_then(Category::from_category_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map_letter_characters_to_letter_category() {
        let upper = 'A';
        let upper_cat = unicode_category_from_char(upper);
        let lower = 'a';
        let lower_cat = unicode_category_from_char(lower);

        assert_eq!(Some(Category::Lu), upper_cat);
        assert_eq!(
            Some(HumanReadableCategory::LetterUppercase),
            upper_cat.map(HumanReadableCategory::from)
        );
        assert_eq!(Some(Category::Ll), lower_cat);
        assert_eq!(
            Some(HumanReadableCategory::LetterLowercase),
            lower_cat.map(HumanReadableCategory::from)
        );
    }

    #[test]
    fn should_map_letter_category_for_extention_trait() {
        assert_eq!(Some(Category::Lu), 'A'.unicode_category());
        assert_eq!(Some(Category::Ll), 'a'.unicode_category());
    }
}
