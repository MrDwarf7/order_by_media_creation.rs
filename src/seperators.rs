#[derive(Clone)]
pub struct Seperators {
    pub date_sep: String,
    pub date_time_sep: String,
    pub time_sep: String,
    pub am_pm_sep: String,
}

impl Seperators {
    pub fn new<S: AsRef<str> + ToString>(
        date_sep: S,
        date_time_sep: S,
        time_sep: S,
        am_pm_sep: S,
    ) -> Self {
        Seperators {
            date_sep: date_sep.to_string(),
            time_sep: time_sep.to_string(),
            date_time_sep: date_time_sep.to_string(),
            am_pm_sep: am_pm_sep.to_string(),
        }
    }
}

impl AsRef<Seperators> for Seperators {
    fn as_ref(&self) -> &Seperators {
        self
    }
}

#[cfg(test)]
mod sep_tests {

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_seperators() {
        let seps = Seperators::new(DATE_SEP, DATE_TIME_SEP, TIME_SEP, AM_PM_SEP);
        assert_eq!(seps.date_sep, "_");
        assert_eq!(seps.date_time_sep, " ");
        assert_eq!(seps.time_sep, ".");
        assert_eq!(seps.am_pm_sep, " ");
    }
}
