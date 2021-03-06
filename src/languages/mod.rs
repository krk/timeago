#![allow(missing_docs)]

#[cfg(feature="isolang")]
extern crate isolang;

pub mod chinese;
pub mod english;
pub mod russian;
pub mod belarusian;
pub mod german;
pub mod polish;
pub mod swedish;
pub mod romanian;
pub mod turkish;


/// Helper function to make a language dynamically dispatched
pub fn boxup<L: super::Language + 'static> (x:L) -> Box<super::Language> {
    Box::new(x) as Box<super::Language>
}

/// A public use for a public dependency.
#[cfg(feature="isolang")]
pub use self::isolang::Language as IsolangLanguage;

/// Try converting a isolang's language into our dynamically dispatched language
/// ```
/// extern crate isolang;
/// extern crate timeago;
/// let il = isolang::Language::from_639_1("ru").unwrap();
/// let l = timeago::from_isolang(il).unwrap();
/// let f = timeago::Formatter::with_language(l);
/// let d = std::time::Duration::from_secs(3600);
/// assert_eq!(f.convert(d), "1 час назад");
/// ```
#[cfg(feature="isolang")]
pub fn from_isolang(x : isolang::Language) -> Option<Box<super::Language>> {
    Some(match x {
        x if x.to_name() == "English"    => boxup(english::English),
        x if x.to_name() == "Chinese"    => boxup(english::Chinese),
        x if x.to_name() == "Russian"    => boxup(russian::Russian),
        x if x.to_name() == "German"     => boxup(german::German),
        x if x.to_name() == "Belarusian" => boxup(belarusian::Belarusian),
        x if x.to_name() == "Polish"     => boxup(polish::Polish),
        x if x.to_name() == "Swedish"    => boxup(swedish::Swedish),
        x if x.to_name() == "Romanian"   => boxup(romanian::Romanian),
        x if x.to_name() == "Turkish"    => boxup(turkish::Turkish),
        _ => return None,
    })
}
