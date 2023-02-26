use std::{borrow::Cow, collections::HashMap};

use twilight_model::{
    application::{
        command::{
            Command, CommandOption, CommandOptionChoice, CommandOptionType, CommandOptionValue,
            CommandType,
        },
        interaction::application_command::InteractionChannel,
    },
    channel::{Attachment, ChannelType},
    guild::{Permissions, Role},
    id::{
        marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker},
        Id,
    },
    user::User,
};

use super::{internal::CreateOptionData, ResolvedUser};

/// Create a slash command from a type.
///
/// This trait is used to create commands from command models. A derive
/// macro is provided to automatically implement the traits.
///
/// ## Types and fields documentation
/// The trait can be derived on structs whose fields implement [`CreateOption`]
/// (see the [module documentation](crate::command) for a list of supported
/// types) or enums whose variants implement [`CreateCommand`].
///
/// Unlike the [`CommandModel`] trait, all fields or variants of the type it's
/// implemented on must have a description. The description corresponds either
/// to the first line of the documentation comment or the value of the `desc`
/// attribute. The type must also be named with the `name` attribute.
///
/// ## Example
/// ```
/// # use twilight_model::guild::Permissions;
/// use twilight_interactions::command::{CreateCommand, ResolvedUser};
///
/// #[derive(CreateCommand)]
/// #[command(
///     name = "hello",
///     desc = "Say hello",
///     default_permissions = "hello_permissions"
/// )]
/// struct HelloCommand {
///     /// The message to send.
///     message: String,
///     /// The user to send the message to.
///     user: Option<ResolvedUser>,
/// }
///
/// fn hello_permissions() -> Permissions {
///     Permissions::SEND_MESSAGES
/// }
/// ```
///
/// ## Macro attributes
/// The macro provides a `#[command]` attribute to provide additional
/// information.
///
/// | Attribute                  | Type                | Location               | Description                                                     |
/// |----------------------------|---------------------|------------------------|-----------------------------------------------------------------|
/// | `name`                     | `str`               | Type                   | Name of the command (required).                                 |
/// | `desc`                     | `str`               | Type / Field / Variant | Description of the command (required).                          |
/// | `default_permissions`      | `fn`[^perms]        | Type                   | Default permissions required by members to run the command.     |
/// | `dm_permission`            | `bool`              | Type                   | Whether the command can be run in DMs.                          |
/// | `nsfw`                     | `bool`              | Type                   | Whether the command is age-restricted.
/// | `rename`                   | `str`               | Field                  | Use a different option name than the field name.                |
/// | `name_localizations`       | `fn`[^localization] | Type / Field / Variant | Localized name of the command (optional).                       |
/// | `desc_localizations`       | `fn`[^localization] | Type / Field / Variant | Localized description of the command (optional).                |
/// | `autocomplete`             | `bool`              | Field                  | Enable autocomplete on this field.                              |
/// | `channel_types`            | `str`               | Field                  | Restricts the channel choice to specific types.[^channel_types] |
/// | `max_value`, `min_value`   | `i64` or `f64`      | Field                  | Set the maximum and/or minimum value permitted.                 |
/// | `max_length`, `min_length` | `u16`               | Field                  |   Maximum and/or minimum string length permitted.               |
///
/// [^perms]: Path to a function that returns [`Permissions`].
///
/// [^localization]: Path to a function that returns a type that implements
/// `IntoIterator<Item = (ToString, ToString)>`. See the module documentation to
/// learn more.
///
/// [^channel_types]: List of [`ChannelType`] names in snake_case separated by spaces
/// like `guild_text private`.
///
/// [`CommandModel`]: super::CommandModel
/// [`ChannelType`]: twilight_model::channel::ChannelType
pub trait CreateCommand: Sized {
    /// Name of the command.
    const NAME: &'static str;

    /// Create an [`ApplicationCommandData`] for this type.
    fn create_command() -> ApplicationCommandData;
}

/// Create a command option from a type.
///
/// This trait is used by the implementation of [`CreateCommand`] generated
/// by the derive macro. See the [module documentation](crate::command) for
/// a list of implemented types.
///
/// ## Option choices
/// This trait can be derived on enums to represent command options with
/// predefined choices. The `#[option]` attribute must be present on each
/// variant.
///
/// ### Example
/// ```
/// use twilight_interactions::command::CreateOption;
///
/// #[derive(CreateOption)]
/// enum TimeUnit {
///     #[option(name = "Minute", value = 60)]
///     Minute,
///     #[option(name = "Hour", value = 3600)]
///     Hour,
///     #[option(name = "Day", value = 86400)]
///     Day,
/// }
/// ```
///
/// ### Macro attributes
/// The macro provides an `#[option]` attribute to configure the generated code.
///
/// | Attribute            | Type                  | Location | Description                                  |
/// |----------------------|-----------------------|----------|----------------------------------------------|
/// | `name`               | `str`                 | Variant  | Set the name of the command option choice.   |
/// | `name_localizations` | `fn`[^localization]   | Variant  | Localized name of the command option choice. |
/// | `value`              | `str`, `i64` or `f64` | Variant  | Value of the command option choice.          |
///
/// [^localization]: Path to a function that returns a type that implements
///                  `IntoIterator<Item = (ToString, ToString)>`. See the
///                  [module documentation](crate::command) to learn more.

pub trait CreateOption: Sized {
    /// Create a [`CommandOption`] from this type.
    fn create_option(data: CreateOptionData) -> CommandOptionExt;
}

/// Wrapper for [`CommandOption`](twilight_model::application::command::CommandOption)
/// to allow more fields.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandOptionExt {
    /// The actual [`CommandOption`](twilight_model::application::command::CommandOption).
    pub inner: CommandOptionExtInner,
    /// Additional optional help.
    pub help: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandOptionExtInner {
    pub autocomplete: Option<bool>,
    pub channel_types: Option<Vec<ChannelType>>,
    pub choices: Option<Vec<CommandOptionChoice>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
    pub kind: CommandOptionType,
    pub max_length: Option<u16>,
    pub max_value: Option<CommandOptionValue>,
    pub min_length: Option<u16>,
    pub min_value: Option<CommandOptionValue>,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub options: Option<Vec<CommandOptionExt>>,
    pub required: Option<bool>,
}

impl From<CommandOptionExtInner> for CommandOption {
    fn from(inner: CommandOptionExtInner) -> Self {
        CommandOption {
            autocomplete: inner.autocomplete,
            channel_types: inner.channel_types,
            choices: inner.choices,
            description: inner.description,
            description_localizations: inner.description_localizations,
            kind: inner.kind,
            max_length: inner.max_length,
            max_value: inner.max_value,
            min_length: inner.min_length,
            min_value: inner.min_value,
            name: inner.name,
            name_localizations: inner.name_localizations,
            options: inner
                .options
                .map(|options| options.into_iter().map(CommandOption::from).collect()),
            required: inner.required,
        }
    }
}

impl From<CommandOptionExt> for CommandOption {
    fn from(o: CommandOptionExt) -> Self {
        o.inner.into()
    }
}

/// Data sent to Discord to create a command.
///
/// This type is used in the [`CreateCommand`] trait.
/// To convert it into a [`Command`], use the [From] (or [Into]) trait.
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationCommandData {
    /// Name of the command. It must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the command name. Keys must be valid
    /// locales.
    pub name_localizations: Option<HashMap<String, String>>,
    /// Description of the command. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the command description. Keys must be valid
    /// locales.
    pub description_localizations: Option<HashMap<String, String>>,
    /// Optional help string. Must not be empty.
    pub help: Option<String>,
    /// List of command options.
    pub options: Vec<CommandOptionExt>,
    /// Whether the command is available in DMs.
    pub dm_permission: Option<bool>,
    /// Default permissions required for a member to run the command.
    pub default_member_permissions: Option<Permissions>,
    /// Whether the command is a subcommand group.
    pub group: bool,
    /// Whether the command is nsfw.
    pub nsfw: Option<bool>,
}

impl From<ApplicationCommandData> for Command {
    fn from(item: ApplicationCommandData) -> Self {
        Command {
            application_id: None,
            guild_id: None,
            name: item.name,
            name_localizations: item.name_localizations,
            default_member_permissions: item.default_member_permissions,
            dm_permission: item.dm_permission,
            description: item.description,
            description_localizations: item.description_localizations,
            id: None,
            kind: CommandType::ChatInput,
            options: item.options.into_iter().map(CommandOption::from).collect(),
            version: Id::new(1),
            nsfw: None,
        }
    }
}

impl From<ApplicationCommandData> for CommandOption {
    fn from(item: ApplicationCommandData) -> Self {
        let data = CreateOptionData {
            name: item.name,
            name_localizations: item.name_localizations,
            description: item.description,
            description_localizations: item.description_localizations,
            required: None,
            autocomplete: false,
            data: Default::default(),
            help: item.help,
        };

        if item.group {
            data.builder(CommandOptionType::SubCommandGroup)
                .options(item.options)
                .build()
                .into()
        } else {
            data.builder(CommandOptionType::SubCommand)
                .options(item.options)
                .build()
                .into()
        }
    }
}

impl From<ApplicationCommandData> for CommandOptionExt {
    fn from(item: ApplicationCommandData) -> Self {
        let inner = CommandOptionExtInner {
            description: item.description,
            name: item.name,
            options: Some(item.options),
            autocomplete: None,
            channel_types: None,
            choices: None,
            description_localizations: None,
            kind: if item.group {
                CommandOptionType::SubCommandGroup
            } else {
                CommandOptionType::SubCommand
            },
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name_localizations: None,
            required: None,
        };

        CommandOptionExt {
            inner,
            help: item.help,
        }
    }
}

impl CreateOption for String {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::String)
    }
}

impl<'a> CreateOption for Cow<'a, str> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::String)
    }
}

macro_rules! impl_for_int {
    ($($ty:ty),*) => {
        $(
            impl CreateOption for $ty {
                fn create_option(data: CreateOptionData) -> CommandOptionExt {
                    data.into_option(CommandOptionType::Integer)
                }
            }
        )*
    }
}

impl_for_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl CreateOption for f64 {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Number)
    }
}

impl CreateOption for f32 {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Number)
    }
}

impl CreateOption for bool {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Boolean)
    }
}

impl CreateOption for Id<UserMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::User)
    }
}

impl CreateOption for Id<ChannelMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Channel)
    }
}

impl CreateOption for Id<RoleMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Role)
    }
}

impl CreateOption for Id<GenericMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Mentionable)
    }
}

impl CreateOption for Id<AttachmentMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Attachment)
    }
}

impl CreateOption for Attachment {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Attachment)
    }
}

impl CreateOption for User {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::User)
    }
}

impl CreateOption for ResolvedUser {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::User)
    }
}

impl CreateOption for InteractionChannel {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Channel)
    }
}

impl CreateOption for Role {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        data.into_option(CommandOptionType::Role)
    }
}
