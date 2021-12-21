#[derive(parse_display::Display, Debug)]
#[display("{0}")]
pub struct UnitName(String);

impl std::str::FromStr for UnitName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut name = String::new();
        for c in s.chars() {
            match c {
                letter @ 'a'..='z' => name.push(letter),
                '@' => name.push_str("_40"),
                _ => return Err(()),
            }
        }

        Ok(UnitName(name))
    }
}

#[derive(parse_display::Display, parse_display::FromStr, Eq, PartialEq, Debug)]
#[display(style = "lowercase")]
pub enum ActiveState {
    /// "active" indicates that unit is active (obviously...).
    Active,
    /// "reloading" indicates that the unit is active and currently reloading its configuration.
    Reloading,
    /// "inactive"indicates that it is inactive and the previous run was successful or no previous run has taken place yet.
    Inactive,
    /// "failed" indicates that it is inactive and the previous run was not successful (more information about the reason for this is available on the unit type specific interfaces, for example for services in the Result property, see below).
    Failed,
    /// "activating" indicates that the unit has previously been inactive but is currently in the process of entering an active state.
    Activating,
    /// "deactivating" indicates that the unit is currently in the process of deactivation.
    Deactivating,
}
