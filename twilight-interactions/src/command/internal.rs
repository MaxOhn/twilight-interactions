//! Internal types used by command traits.
//!
//! This module contains types used in traits definitions of the [`command`] module
//! and used by implementations generated by the derive macros.
//!
//! <pre class="compile_fail" style="white-space:normal;font:inherit;">
//!     <strong>Warning</strong>: Types exposed by this modules are not intended
//!     to be used directly, and thus do not respect semantic versioning. Breaking
//!     changes may occur in minor version bumps.
//! </pre>
//!
//! [`command`]: crate::command

use twilight_model::{
    application::command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
        CommandOption, CommandOptionChoice, CommandOptionValue, CommandType,
        NumberCommandOptionData, OptionsCommandOptionData,
    },
    channel::ChannelType,
    id::CommandVersionId,
};

/// Data of a command option.
///
/// This type is used in the [`CreateOption`] trait.
///
/// <pre class="compile_fail" style="white-space:normal;font:inherit;">
///     <strong>Warning</strong>: This type is not intended to be used directly,
///     and thus do not respect semantic versioning. Read
///     <a href="index.html">module documentation</a> to learn more.
/// </pre>
///
/// [`CreateOption`]: super::CreateOption
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOptionData {
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Whether the option is required to be completed by a user.
    pub required: bool,
    /// Whether the command supports autocomplete. Only for `STRING`, `INTEGER` and `NUMBER` option type.
    pub autocomplete: bool,
    /// Restricts the channel choice to specific types. Only for `CHANNEL` option type.
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted. Only for `INTEGER` and `NUMBER` option type.
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted. Only for `INTEGER` and `NUMBER` option type.
    pub min_value: Option<CommandOptionValue>,
}

impl CommandOptionData {
    /// Conversion into a [`BaseCommandOptionData`]
    pub fn into_data(self) -> BaseCommandOptionData {
        BaseCommandOptionData {
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChannelCommandOptionData`]
    pub fn into_channel(self) -> ChannelCommandOptionData {
        ChannelCommandOptionData {
            channel_types: self.channel_types,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChoiceCommandOptionData`]
    pub fn into_choice(self, choices: Vec<CommandOptionChoice>) -> ChoiceCommandOptionData {
        ChoiceCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`NumberCommandOptionData`]
    pub fn into_number(self, choices: Vec<CommandOptionChoice>) -> NumberCommandOptionData {
        NumberCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            max_value: self.max_value,
            min_value: self.min_value,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`OptionsCommandOptionData`]
    pub fn into_options(self, options: Vec<CommandOption>) -> OptionsCommandOptionData {
        OptionsCommandOptionData {
            description: self.description,
            name: self.name,
            options,
        }
    }
}

/// Data sent to discord to create a command.
///
/// This type is used in the implementation of the [`CreateCommand`]
/// trait generated by derive macro.
///
/// <pre class="compile_fail" style="white-space:normal;font:inherit;">
///     <strong>Warning</strong>: This type is not intended to be used directly,
///     and thus do not respect semantic versioning. Read
///     <a href="index.html">module documentation</a> to learn more.
/// </pre>
///
/// [`CreateCommand`]: super::CreateCommand
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCommandData {
    /// Name of the command. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// List of command options.
    pub options: Vec<CommandOption>,
    /// Whether the command is enabled by default when the app is added to a guild.
    pub default_permission: bool,
}

impl From<ApplicationCommandData> for Command {
    fn from(item: ApplicationCommandData) -> Self {
        Command {
            application_id: None,
            guild_id: None,
            name: item.name,
            default_permission: Some(item.default_permission),
            description: item.description,
            id: None,
            kind: CommandType::ChatInput,
            options: item.options,
            version: CommandVersionId::new(1).unwrap(),
        }
    }
}
