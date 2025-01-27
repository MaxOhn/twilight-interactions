//! Internal types used by command traits.
//!
//! This module contains types used by trait definitions in the [`command`]
//! module and implementations generated by the derive macros.
//!
//! [`command`]: crate::command

use std::collections::HashMap;

use twilight_model::{
    application::command::{CommandOptionChoice, CommandOptionType, CommandOptionValue},
    channel::ChannelType,
};

use super::{create_command::CommandOptionExtended, DescLocalizations, NameLocalizations};

/// Internal representation of localization types ([`NameLocalizations`] and
/// [`DescLocalizations`]).
#[derive(Debug, Clone, PartialEq)]
pub struct LocalizationsInternal {
    pub fallback: String,
    pub localizations: Option<HashMap<String, String>>,
}

/// Convert a type into a [`LocalizationsInternal`].
pub trait IntoLocalizationsInternal {
    fn into_localizations(self) -> LocalizationsInternal;
}

impl IntoLocalizationsInternal for DescLocalizations {
    fn into_localizations(self) -> LocalizationsInternal {
        LocalizationsInternal {
            fallback: self.fallback,
            localizations: Some(self.localizations),
        }
    }
}

impl IntoLocalizationsInternal for (&str, Option<NameLocalizations>) {
    fn into_localizations(self) -> LocalizationsInternal {
        LocalizationsInternal {
            fallback: self.0.to_owned(),
            localizations: self.1.map(|v| v.localizations),
        }
    }
}

impl IntoLocalizationsInternal for &str {
    fn into_localizations(self) -> LocalizationsInternal {
        LocalizationsInternal {
            fallback: self.to_owned(),
            localizations: None,
        }
    }
}

/// Data to create a command option from.
///
/// This type is used in the [`CreateOption`] trait and contains a subset of
/// twilight's [`CommandOption`] fields.
///
/// [`CreateOption`]: super::CreateOption
#[derive(Debug, Clone, PartialEq)]
pub struct CreateOptionData {
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the option name. Keys must be valid locales.
    pub name_localizations: Option<HashMap<String, String>>,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the option description. Keys must be valid
    /// locales.
    pub description_localizations: Option<HashMap<String, String>>,
    /// Optional help. Must not be empty.
    pub help: Option<&'static str>,
    /// Whether the option is required to be completed by a user.
    pub required: Option<bool>,
    /// Whether the command supports autocomplete. Only for `STRING`, `INTEGER`
    /// and `NUMBER` option types.
    pub autocomplete: bool,
    /// Data of the command option.
    pub data: CommandOptionData,
}

/// Data of a command option.
///
/// This type holds settings of a command option used when
/// parsing the option. It is used in the [`CommandOption`]
/// trait.
///
/// [`CommandOption`]: super::CommandOption
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommandOptionData {
    /// Restricts the channel choice to specific types. Only for `CHANNEL`
    /// option type.
    pub channel_types: Option<Vec<ChannelType>>,
    /// Maximum value permitted. Only for `INTEGER` and `NUMBER` option types.
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted. Only for `INTEGER` and `NUMBER` option types.
    pub min_value: Option<CommandOptionValue>,
    /// Minimum value length. Only for `STRING` option type.
    pub max_length: Option<u16>,
    /// Maximum value length. Only for `STRING` option type.
    pub min_length: Option<u16>,
}

/// Builder to convert a [`CreateOptionData`] into a [`CommandOption`].
pub struct CreateOptionBuilder {
    kind: CommandOptionType,
    option: CreateOptionData,
    choices: Option<Vec<CommandOptionChoice>>,
    options: Option<Vec<CommandOptionExtended>>,
}

impl CreateOptionBuilder {
    /// Create a new [`CreateOptionBuilder`].
    pub fn new(option: CreateOptionData, kind: CommandOptionType) -> Self {
        Self {
            kind,
            option,
            choices: None,
            options: None,
        }
    }

    /// Set the option choices.
    pub fn choices(mut self, choices: Vec<CommandOptionChoice>) -> Self {
        self.choices = Some(choices);

        self
    }

    /// Set the subcommand options.
    pub fn options(mut self, options: Vec<CommandOptionExtended>) -> Self {
        self.options = Some(options);

        self
    }

    /// Build the [`CommandOption`].
    pub fn build(self) -> CommandOptionExtended {
        CommandOptionExtended {
            autocomplete: Some(self.option.autocomplete),
            channel_types: self.option.data.channel_types,
            choices: self.choices,
            description: self.option.description,
            description_localizations: self.option.description_localizations,
            kind: self.kind,
            max_length: self.option.data.max_length,
            max_value: self.option.data.max_value,
            min_length: self.option.data.min_length,
            min_value: self.option.data.min_value,
            name: self.option.name,
            name_localizations: self.option.name_localizations,
            help: self.option.help,
            options: self.options,
            required: self.option.required,
        }
    }
}

impl CreateOptionData {
    /// Create a new [`CreateOptionBuilder`].
    pub fn builder(self, kind: CommandOptionType) -> CreateOptionBuilder {
        CreateOptionBuilder::new(self, kind)
    }

    /// Convert the data into a [`CommandOption`].
    pub fn into_option(self, kind: CommandOptionType) -> CommandOptionExtended {
        self.builder(kind).build()
    }
}
