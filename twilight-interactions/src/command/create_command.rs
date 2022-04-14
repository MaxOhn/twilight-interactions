use std::borrow::Cow;

use twilight_model::{
    application::{
        command::{
            BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
            CommandOption, CommandType, Number, NumberCommandOptionData, OptionsCommandOptionData,
        },
        interaction::application_command::InteractionChannel,
    },
    channel::Attachment,
    guild::Role,
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
/// The trait can be derived structs where all fields implement [`CreateOption`]
/// (see the [module documentation](crate::command) for a list of supported types)
/// or enums where variants implements [`CreateCommand`].
///
/// Unlike the [`CommandModel`] trait, the type its field or variants must have
/// a description. The description correspond either to the first line of the
/// documentation comment, or the value of the `desc` attribute. The type must
/// also be named with the `name` attribute.
///
/// ## Example
/// ```
/// use twilight_interactions::command::{CreateCommand, ResolvedUser};
///
/// #[derive(CreateCommand)]
/// #[command(name = "hello", desc = "Say hello")]
/// struct HelloCommand {
///     /// The message to send.
///     message: String,
///     /// The user to send the message to.
///     user: Option<ResolvedUser>,
/// }
/// ```
///
/// ## Macro attributes
/// The macro provide a `#[command]` attribute to provide additional information.
///
/// | Attribute                | Type           | Location               | Description                                                     |
/// |--------------------------|----------------|------------------------|-----------------------------------------------------------------|
/// | `name`                   | `str`          | Type                   | Name of the command (required).                                 |
/// | `desc`                   | `str`          | Type / Field / Variant | Set the subcommand name (required).                             |
/// | `default_permission`     | `bool`         | Type                   | Whether the command should be enabled by default.               |
/// | `rename`                 | `str`          | Field                  | Use a different option name than the field name.                |
/// | `autocomplete`           | `bool`         | Field                  | Enable autocomplete on this field.                              |
/// | `channel_types`          | `str`          | Field                  | Restricts the channel choice to specific types.[^channel_types] |
/// | `max_value`, `min_value` | `i64` or `f64` | Field                  | Set the maximum and/or minimum value permitted.                 |
///
/// ## Example
/// ```
/// use twilight_interactions::command::{CreateCommand, ResolvedUser};
///
/// #[derive(CreateCommand)]
/// #[command(name = "hello", desc = "Say hello")]
/// struct HelloCommand {
///     /// The message to send.
///     message: String,
///     /// The user to send the message to.
///     user: Option<ResolvedUser>,
/// }
/// ```
///
/// [^channel_types]: List [`ChannelType`] names in snake_case separated by spaces
///                   like `guild_text private`.
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
///     Day
/// }
/// ```
///
/// ### Macro attributes
/// The macro provide a `#[option]` attribute to configure the generated code.
///
/// | Attribute | Type                  | Location | Description                                |
/// |-----------|-----------------------|----------|--------------------------------------------|
/// | `name`    | `str`                 | Variant  | Set the name of the command option choice. |
/// | `value`   | `str`, `i64` or `f64` | Variant  | Value of the command option choice.        |

pub trait CreateOption: Sized {
    /// Create a [`CommandOption`] from this type.
    fn create_option(data: CreateOptionData) -> CommandOptionExt;
}

/// Wrapper for [`CommandOption`](twilight_model::application::command::CommandOption)
/// to allow more fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOptionExt {
    /// The actual [`CommandOption`](twilight_model::application::command::CommandOption).
    pub inner: CommandOptionExtInner,
    /// Additional optional help.
    pub help: Option<String>,
}

/// Inner option for [`CommandOptionExt`] to distinguish
/// between [`CommandOption`]'s variants for subcommands(groups).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOptionExtInner {
    SubCommand(OptionsCommandOptionDataExt),
    SubCommandGroup(OptionsCommandOptionDataExt),
    String(ChoiceCommandOptionData),
    Integer(NumberCommandOptionData),
    Boolean(BaseCommandOptionData),
    User(BaseCommandOptionData),
    Channel(ChannelCommandOptionData),
    Role(BaseCommandOptionData),
    Mentionable(BaseCommandOptionData),
    Number(NumberCommandOptionData),
    Attachment(BaseCommandOptionData),
}

impl CommandOptionExtInner {
    pub fn name(&self) -> &str {
        match self {
            CommandOptionExtInner::SubCommand(d) => d.name.as_str(),
            CommandOptionExtInner::SubCommandGroup(d) => d.name.as_str(),
            CommandOptionExtInner::String(d) => d.name.as_str(),
            CommandOptionExtInner::Integer(d) => d.name.as_str(),
            CommandOptionExtInner::Boolean(d) => d.name.as_str(),
            CommandOptionExtInner::User(d) => d.name.as_str(),
            CommandOptionExtInner::Channel(d) => d.name.as_str(),
            CommandOptionExtInner::Role(d) => d.name.as_str(),
            CommandOptionExtInner::Mentionable(d) => d.name.as_str(),
            CommandOptionExtInner::Number(d) => d.name.as_str(),
            CommandOptionExtInner::Attachment(d) => d.name.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsCommandOptionDataExt {
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Used for specifying the nested options in a subcommand(group).
    pub options: Vec<CommandOptionExt>,
}

impl From<OptionsCommandOptionDataExt> for OptionsCommandOptionData {
    fn from(o: OptionsCommandOptionDataExt) -> Self {
        Self {
            description: o.description,
            name: o.name,
            options: o.options.into_iter().map(CommandOption::from).collect(),
        }
    }
}

impl From<CommandOptionExtInner> for CommandOption {
    fn from(inner: CommandOptionExtInner) -> Self {
        match inner {
            CommandOptionExtInner::SubCommand(d) => CommandOption::SubCommand(d.into()),
            CommandOptionExtInner::SubCommandGroup(d) => CommandOption::SubCommandGroup(d.into()),
            CommandOptionExtInner::String(d) => CommandOption::String(d),
            CommandOptionExtInner::Integer(d) => CommandOption::Integer(d),
            CommandOptionExtInner::Boolean(d) => CommandOption::Boolean(d),
            CommandOptionExtInner::User(d) => CommandOption::User(d),
            CommandOptionExtInner::Channel(d) => CommandOption::Channel(d),
            CommandOptionExtInner::Role(d) => CommandOption::Role(d),
            CommandOptionExtInner::Mentionable(d) => CommandOption::Mentionable(d),
            CommandOptionExtInner::Number(d) => CommandOption::Number(d),
            CommandOptionExtInner::Attachment(d) => CommandOption::Attachment(d),
        }
    }
}

impl From<CommandOptionExt> for CommandOption {
    fn from(o: CommandOptionExt) -> Self {
        o.inner.into()
    }
}

/// Data sent to discord to create a command.
///
/// This type is used in the [`CreateCommand`] trait.
/// To convert it into a [`Command`], use the [From] (or [Into]) trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCommandData {
    /// Name of the command. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Optional help string. Must not be empty.
    pub help: Option<String>,
    /// List of command options.
    pub options: Vec<CommandOptionExt>,
    /// Whether the command is enabled by default when the app is added to a guild.
    pub default_permission: bool,
    /// Whether the command is a subcommand group.
    pub group: bool,
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
            options: item.options.into_iter().map(CommandOption::from).collect(),
            version: Id::new(1),
        }
    }
}

impl From<ApplicationCommandData> for CommandOption {
    fn from(item: ApplicationCommandData) -> Self {
        let data = OptionsCommandOptionData {
            description: item.description,
            name: item.name,
            options: item.options.into_iter().map(CommandOption::from).collect(),
        };

        if item.group {
            CommandOption::SubCommandGroup(data)
        } else {
            CommandOption::SubCommand(data)
        }
    }
}

impl From<ApplicationCommandData> for CommandOptionExt {
    fn from(item: ApplicationCommandData) -> Self {
        let data = OptionsCommandOptionDataExt {
            description: item.description,
            name: item.name,
            options: item.options,
        };

        let inner = if item.group {
            CommandOptionExtInner::SubCommandGroup(data)
        } else {
            CommandOptionExtInner::SubCommand(data)
        };

        CommandOptionExt {
            inner,
            help: item.help,
        }
    }
}

impl CreateOption for String {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_choice(Vec::new());

        CommandOptionExt {
            inner: CommandOptionExtInner::String(opt),
            help,
        }
    }
}

impl<'a> CreateOption for Cow<'a, str> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_choice(Vec::new());

        CommandOptionExt {
            inner: CommandOptionExtInner::String(opt),
            help,
        }
    }
}

macro_rules! impl_for_int {
    ($($ty:ty),*) => {
        $(
            impl CreateOption for $ty {
                fn create_option(data: CreateOptionData) -> CommandOptionExt {
                    let (opt, help) = data.into_number(Vec::new());

                    CommandOptionExt {
                        inner: CommandOptionExtInner::Integer(opt),
                        help,
                    }
                }
            }
        )*
    }
}

impl_for_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl CreateOption for Number {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_number(Vec::new());

        CommandOptionExt {
            inner: CommandOptionExtInner::Number(opt),
            help,
        }
    }
}

impl CreateOption for f64 {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_number(Vec::new());

        CommandOptionExt {
            inner: CommandOptionExtInner::Number(opt),
            help,
        }
    }
}

impl CreateOption for f32 {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_number(Vec::new());

        CommandOptionExt {
            inner: CommandOptionExtInner::Number(opt),
            help,
        }
    }
}

impl CreateOption for bool {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::Boolean(opt),
            help,
        }
    }
}

impl CreateOption for Id<UserMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::User(opt),
            help,
        }
    }
}

impl CreateOption for Id<ChannelMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_channel();

        CommandOptionExt {
            inner: CommandOptionExtInner::Channel(opt),
            help,
        }
    }
}

impl CreateOption for Id<RoleMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::Role(opt),
            help,
        }
    }
}

impl CreateOption for Id<GenericMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::Mentionable(opt),
            help,
        }
    }
}

impl CreateOption for Id<AttachmentMarker> {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::Attachment(opt),
            help,
        }
    }
}

impl CreateOption for Attachment {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::Attachment(opt),
            help,
        }
    }
}

impl CreateOption for User {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::User(opt),
            help,
        }
    }
}

impl CreateOption for ResolvedUser {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::User(opt),
            help,
        }
    }
}

impl CreateOption for InteractionChannel {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_channel();

        CommandOptionExt {
            inner: CommandOptionExtInner::Channel(opt),
            help,
        }
    }
}

impl CreateOption for Role {
    fn create_option(data: CreateOptionData) -> CommandOptionExt {
        let (opt, help) = data.into_data();

        CommandOptionExt {
            inner: CommandOptionExtInner::Role(opt),
            help,
        }
    }
}
