//! Roles: the frame's emphasis vocabulary. One name per visual job, so no
//! renderer ever picks a raw SGR code and every theme answers the same set.
//! Adding a role means one entry here and one code per theme; the
//! completeness test fails until every theme answers it.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    /// Identity and screen titles.
    Title,
    /// The primary voice: the active harness, a section name.
    Accent,
    /// The second voice: the converse reply side.
    Accent2,
    /// Field labels and commands.
    Label,
    /// Field values and body text that is not dimmed.
    Value,
    /// The greyed chrome layer: rules, hints, secondary text.
    Dim,
    /// A good outcome.
    Ok,
    /// Something to notice.
    Warn,
    /// Something to fix.
    Err,
    /// Section and separator rules.
    Rule,
}

/// The number of roles; the palette table is sized by it.
pub const COUNT: usize = 10;

/// Every role, in table order -- the completeness test walks this.
#[cfg(test)]
pub const ALL: [Role; COUNT] = [
    Role::Title,
    Role::Accent,
    Role::Accent2,
    Role::Label,
    Role::Value,
    Role::Dim,
    Role::Ok,
    Role::Warn,
    Role::Err,
    Role::Rule,
];
