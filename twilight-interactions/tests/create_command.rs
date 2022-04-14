use twilight_interactions::command::{
    ApplicationCommandData, CommandOptionExt, CommandOptionExtInner, CreateCommand, ResolvedUser,
};
use twilight_model::{
    application::{
        command::{
            BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData,
            CommandOptionValue, Number, NumberCommandOptionData,
        },
        interaction::application_command::InteractionChannel,
    },
    channel::ChannelType,
};

/// Demo command for testing purposes
#[derive(CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "demo", help = "more demo")]
struct DemoCommand {
    /// This should be overwritten
    #[command(rename = "member", desc = "A member", help = "abc")]
    user: ResolvedUser,
    /// Some text
    ///
    /// This documentation comment is ignored
    text: String,
    /// A number
    #[command(autocomplete = true, max_value = 50.0)]
    number: Number,
    /// A text channel
    #[command(channel_types = "guild_text private")]
    channel: Option<InteractionChannel>,
}

#[derive(CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "unit", desc = "Unit command for testing purposes")]
struct UnitCommand;

#[test]
fn test_create_command() {
    let options = vec![
        CommandOptionExt {
            inner: CommandOptionExtInner::User(BaseCommandOptionData {
                description: "A member".into(),
                name: "member".into(),
                required: true,
            }),
            help: Some("abc".to_owned()),
        },
        CommandOptionExt {
            inner: CommandOptionExtInner::String(ChoiceCommandOptionData {
                autocomplete: false,
                description: "Some text".into(),
                name: "text".into(),
                required: true,
                choices: vec![],
            }),
            help: None,
        },
        CommandOptionExt {
            inner: CommandOptionExtInner::Number(NumberCommandOptionData {
                autocomplete: true,
                choices: vec![],
                description: "A number".into(),
                max_value: Some(CommandOptionValue::Number(Number(50.0))),
                min_value: None,
                name: "number".into(),
                required: true,
            }),
            help: None,
        },
        CommandOptionExt {
            inner: CommandOptionExtInner::Channel(ChannelCommandOptionData {
                channel_types: vec![ChannelType::GuildText, ChannelType::Private],
                description: "A text channel".into(),
                name: "channel".into(),
                required: false,
            }),
            help: None,
        },
    ];

    let expected = ApplicationCommandData {
        name: "demo".into(),
        description: "Demo command for testing purposes".into(),
        help: Some("more demo".into()),
        options,
        default_permission: true,
        group: false,
    };

    assert_eq!(DemoCommand::create_command(), expected);
    assert_eq!(DemoCommand::NAME, "demo");
}

#[test]
fn test_unit_create_command() {
    let expected = ApplicationCommandData {
        name: "unit".into(),
        description: "Unit command for testing purposes".into(),
        help: None,
        options: vec![],
        default_permission: true,
        group: false,
    };

    assert_eq!(UnitCommand::create_command(), expected);
    assert_eq!(UnitCommand::NAME, "unit");
}
