use std::{fs::{read_to_string, create_dir_all}, path::PathBuf};
use structstruck::strike;
use serde_derive::*;
use log::error;
use serenity::{
    json,
    builder::CreateApplicationCommand,
    model::{
        prelude::{command::CommandOptionType, GuildId, User},
        Permissions,
    },
    prelude::Context,
};
use chrono::prelude::*;


strike! {
    #[strikethrough[derive(Deserialize, Serialize, Debug, Clone, Default)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct Warnings
    {
        /// All the guilds with users that have warnings
        guilds: Vec<pub struct GuildWarnings
        {
            /// The guild's ID
            id: GuildId

            /// All the users with warnings in the guild
            users: Vec<pub struct UserWarnings
            {
                /// The user's ID
                user: User,

                /// How many warnings the user has
                warning_count: u32,

                /// An array of warnings
                warnings: Vec<pub struct Warning
                {
                    /// Reason for the warning
                    reason: String,
                }>
                
            }>
        }>,
        
    }
}

impl Warning
{
    pub fn new(reason: String) -> Self
    {
        Self{reason}
    }
}

impl GuildWarnings
{
    pub fn new(id: GuildId) -> Self
    {
        Self{
            id,
            ..Default::default()
        }
    }

    pub fn push_user(&mut self, user: UserWarnings)
    {
        self.users.push(user);
        self
    }
}

impl ToString for UserWarnings
{
    fn to_string(&self) -> String
    {
        let mut buffer = format!(
            "User {} has {} warnings",
            self.user.name,
            self.warning_count,
        )

        if self.warning_count > 0
        {
            buffer.push_str(":\n");
            
            for (i, warning) in self.warnings.clone().iter().enumerate()
            {
                buffer.push_str(&format!("Warning {}: {}\n", i, warning.reason));
            }
        }

        buffer
    } 
}

impl UserWarnings
{
    /// Create a new `UserWarnings` structure
    pub fn new(id: UserId) -> Self
    {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn push_warning(&mut self, reason: String) -> Self
    {
        // Push the warning
        self.warnings.push(Warning::new(reason));
        
        // Increment the warning counter
        self.warning_count +=1;
        self
    }
}

impl Warnings
{
    pub fn save(&self, path: PathBuf) -> std::io::Result<()>
    {

        // If there's a parent to this path, ensure it exists 
        if let Some(parent) = path.parent()
        {
            if !parent.exits()
           {
             create_dir_all(parent)?;
           }
        }

        let serialized = serde_json::to_string(&self?).unwrap();
        fs::write(path,serialized)?;
    }
    
    pub fn load(path: PathBuf) -> Result<Self, Error> 
    {

    }

    pub fn add_warning(&mut self, guild_id, user_id, reason: String)
    {
        // Search for where the guild we're looking for is
        let guild_pos = match self.guilds.clone().iter().position(|&g| g.id == guild_id)
        {
            Some(x) => x,

            // Create a new guild struct and push it
            None => {
                

                // Creae a new guild struct and push the user to it
                let new_guild = GuildWarnings::new(guild_id).push_user(new_user);

                // Add the guild to the record
                self.guilds.push(new_guild);

                // Return the index to last item in the array. This should be the
                // guild we just pushed.
                self.guilds.len() -1
            },
        };

        let guild_warnings = self.guilds[guild_pos].clone();

        // Search for where our user is
        let user_pos = match guild_warnings.iter().position(|&u| u.id == user_id)
        {
            Some(x) =>x,

            // Create a new user and push it
            None => {

                // Create a new `UserWarnings` and push the warning to it
                let new_user = UserWarnings::new(user_id).push_warning(reason);
                
                // Push the `UserWarnings` to the guild.
                self.guilds[guild_pos].push_user(new_user);
            },
        }


    }

    pub fn get_warnings(
        &self,
        guild_id: GuildId,
        user: User
    ) -> Option<UserWarnings>
    {
        // Search for where the guild we're looking for is
        let guild_pos = match self.guilds.clone().iter().position(|&g| g.id == guild_id)
        {
            Some(x) => x,
            None => return None,
        };

        // Get our guild
        let guild_warnings = self.guilds[guild_pos].clone();

        // Search for where our user is
        let user_pos = match guild_warnings.iter().position(|&u| u.user == user)
        {
            Some(x) =>x,
            None => return None,
        }

        /// Return the User's warning information
        Some(self.guilds[guild_pos].users[user_pos])
        
    }
}