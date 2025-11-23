use serde::{Deserialize, Serialize};

/// The group the user belongs to, for example `User`, `Moderator`...
///
/// For more informations, check out [the website](https://wallhaven.cc/team)
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum UserGroup {
    /// A normal user, the group you will be assigned to on account creation
    User,
    /// A moderator
    Moderator,
    /// A senior moderator
    ///
    /// There is not a lot of information on how to get this role,
    /// I'm guessing you get this from being a moderator for enough time
    #[serde(rename = "Senior Moderator")]
    SeniorModerator,
    /// An administrator
    ///
    /// There is not a lot of information on how to get this role
    Administrator,
    /// One of the website's developers
    Developer,
    /// The owner (and main developer) of the website
    ///
    /// The only user with this role should be [AksumkA](https://wallhaven.cc/user/AksumkA)
    #[serde(rename = "Owner/Developer")]
    Owner,
}
