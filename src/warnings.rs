

use serde_derive::*;
use serenity::{
    model::{
        prelude::{GuildId, User},
    },
};
use std::{
    fs::{self, create_dir_all, read_to_string},
    path::PathBuf,
};
use structstruck::strike;

strike! {
    #[strikethrough[derive(Serialize,Deserialize, Debug, Clone, Default, PartialEq,)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct Warnings
    {
        /// All the guilds with users that have warnings
        pub guilds: Vec<pub struct GuildWarnings
        {
            /// The guild's ID
            pub id: GuildId,

            /// All the users with warnings in the guild
            pub users: Vec<pub struct UserWarnings
            {
                /// The user
                pub user: User,

                /// How many warnings the user has
                pub warning_count: u32,

                /// An array of warnings
                pub warnings: Vec<pub struct Warning
                {
                    /// Reason for the warning
                    pub reason: String,
                }>

            }>
        }>,

    }
}

impl Warning
{
    pub fn new(reason: String) -> Self { Self { reason } }
}

impl GuildWarnings
{
    pub fn new(id: GuildId) -> Self
    {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl ToString for UserWarnings
{
    fn to_string(&self) -> String
    {
        let mut buffer = format!(
            "User {} has {} warning(s)",
            self.user.name, self.warning_count,
        );

        if self.warning_count > 0
        {
            buffer.push_str(":\n");

            for (i, warning) in self.warnings.clone().iter().enumerate()
            {
                buffer.push_str(&format!("{}: {}\n", i +1, warning.reason));
            }
        }

        buffer
    }
}

impl UserWarnings
{
    /// Create a new `UserWarnings` structure
    pub fn new(user: User) -> Self
    {
        Self {
            user,
            ..Default::default()
        }
    }
}

impl Warnings
{
    /// Create an empty `Warnings`
    pub fn new() -> Self { Default::default() }

    /// Save the warnings to a file
    pub fn save(&self, path: PathBuf) -> std::io::Result<()>
    {
        // If there's a parent to this path, ensure it exists
        if let Some(parent) = path.parent()
        {
            if !parent.exists()
            {
                create_dir_all(parent)?;
            }
        }

        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(path, serialized)?;

        Ok(())
    }

    /// Load the warnings to a file
    pub fn load(path: &PathBuf) -> Result<Self, String>
    {
        if !path.is_file()
        {
            return Err(format!("Error: No file '{}' found", path.display()));
        }

        if let Ok(s) = read_to_string(path)
        {
            let ret = match serde_json::from_str(&s)
            {
                Ok(x) => x,
                Err(x) => return Err(format!("Error: {}", x)),
            };
            Ok(ret)
        }
        else
        {
            Err(format!(
                "Error: failed reading from file: {}",
                path.display()
            ))
        }
    }

    /// Add a new warning to a user
    pub fn add_warning(&mut self, guild_id: &GuildId, user: User, reason: String)
    {
        // Search for where the guild we're looking for is
        let guild_pos = match self
            .guilds
            .clone()
            .iter()
            .position(|g| g.id.to_string() == guild_id.to_string())
        {
            Some(x) => x,

            // Create a new guild struct and push it
            None =>
            {
                // Creae a new guild struct and push the user to it
                let new_guild = GuildWarnings::new(*guild_id);

                // Add the guild to the record
                self.guilds.push(new_guild);

                // Return the index to last item in the array. This should be the
                // guild we just pushed.
                self.guilds.len() - 1
            }
        };

        let guild_warnings = self.guilds[guild_pos].clone();

        // Search for where our user is
        let user_pos = match guild_warnings
            .users
            .iter()
            .position(|u| u.user.id.to_string() == user.id.to_string())
        {
            Some(x) => x,

            // Create a new user and push it
            None =>
            {
                // Create a new `UserWarnings` and push the warning to it
                let new_user = UserWarnings::new(user);

                // Push the `UserWarnings` to the guild.
                self.guilds[guild_pos].users.push(new_user);
                self.guilds[guild_pos].users.len() - 1
            }
        };

        // Push the warning
        self.guilds[guild_pos].users[user_pos]
            .warnings
            .push(Warning::new(reason));
        // Increment the warning counter
        self.guilds[guild_pos].users[user_pos].warning_count += 1;
    }

    /// Get a vector of a user's warnings
    pub fn get_warnings(&self, guild_id: GuildId, user: User) -> Option<UserWarnings>
    {
        // Search for where the guild we're looking for is
        let guild_pos = match self.guilds.clone().iter().position(|g| g.id == guild_id)
        {
            Some(x) => x,
            None => return None,
        };

        // Get our guild
        let guild_warnings = self.guilds[guild_pos].clone();

        // Search for where our user is
        let user_pos = match guild_warnings.users.iter().position(|u| u.user == user)
        {
            Some(x) => x,
            None => return None,
        };

        // Return the User's warning information
        Some(self.guilds[guild_pos].users[user_pos].clone())
    }
}
