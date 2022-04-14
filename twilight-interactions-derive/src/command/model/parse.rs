//! Parsing of struct fields and attributes

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Attribute, Error, Lit, Result, Type};

use crate::parse::{
    extract_option, find_attr, parse_desc, parse_help, parse_name, AttrValue, NamedAttrs,
};

/// Parsed struct field
pub struct StructField {
    pub span: Span,
    pub ident: Ident,
    pub ty: Type,
    pub raw_attrs: Vec<Attribute>,
    pub attributes: FieldAttribute,
    pub kind: FieldType,
}

/// Type of a parsed struct field
pub enum FieldType {
    Required,
    Optional,
}

impl StructField {
    /// Parse a [`syn::Field`] as a [`StructField`]
    pub fn from_field(field: syn::Field) -> Result<Self> {
        let (kind, ty) = match extract_option(&field.ty) {
            Some(ty) => (FieldType::Optional, ty),
            None => (FieldType::Required, field.ty.clone()),
        };

        let attributes = match find_attr(&field.attrs, "command") {
            Some(attr) => FieldAttribute::parse(attr)?,
            None => FieldAttribute::default(),
        };

        Ok(Self {
            span: field.ty.span(),
            ident: field.ident.unwrap(),
            ty,
            raw_attrs: field.attrs,
            attributes,
            kind,
        })
    }

    /// Parse [`syn::FieldsNamed`] as a [`Vec<StructField>`]
    pub fn from_fields(fields: syn::FieldsNamed) -> Result<Vec<Self>> {
        fields.named.into_iter().map(Self::from_field).collect()
    }
}

impl FieldType {
    pub fn required(&self) -> bool {
        match self {
            FieldType::Required => true,
            FieldType::Optional => false,
        }
    }
}

/// Parsed type attribute
pub struct TypeAttribute {
    /// Rename the field to the given name
    pub name: Option<String>,
    /// Overwrite the field description
    pub desc: Option<String>,
    /// Optional help
    pub help: Option<String>,
    /// Whether the command should be enabled by default.
    pub default_permission: bool,
}

impl TypeAttribute {
    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["name", "desc", "default_permission", "help"])?;

        let name = attrs.get("name").map(parse_name).transpose()?;
        let desc = attrs.get("desc").map(parse_desc).transpose()?;
        let help = attrs.get("help").map(parse_help).transpose()?;
        let default_permission = attrs
            .get("default_permission")
            .map(|v| v.parse_bool())
            .transpose()?
            .unwrap_or(true);

        Ok(Self {
            name,
            desc,
            help,
            default_permission,
        })
    }
}

/// Parsed field attribute
#[derive(Default)]
pub struct FieldAttribute {
    /// Rename the field to the given name
    pub rename: Option<String>,
    /// Overwrite the field description
    pub desc: Option<String>,
    /// Optional help
    pub help: Option<String>,
    /// Whether the field supports autocomplete
    pub autocomplete: bool,
    /// Limit to specific channel types
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted
    pub min_value: Option<CommandOptionValue>,
}

impl FieldAttribute {
    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(
            meta,
            &[
                "rename",
                "desc",
                "help",
                "autocomplete",
                "channel_types",
                "max_value",
                "min_value",
            ],
        )?;

        let rename = attrs.get("rename").map(parse_name).transpose()?;
        let desc = attrs.get("desc").map(parse_desc).transpose()?;
        let help = attrs.get("help").map(parse_help).transpose()?;
        let autocomplete = attrs
            .get("autocomplete")
            .map(|val| val.parse_bool())
            .transpose()?
            .unwrap_or_default();
        let channel_types = attrs
            .get("channel_types")
            .map(ChannelType::parse_attr)
            .transpose()?
            .unwrap_or_default();
        let max_value = attrs
            .get("max_value")
            .map(CommandOptionValue::parse_attr)
            .transpose()?;
        let min_value = attrs
            .get("min_value")
            .map(CommandOptionValue::parse_attr)
            .transpose()?;

        Ok(Self {
            rename,
            desc,
            help,
            autocomplete,
            channel_types,
            max_value,
            min_value,
        })
    }

    pub fn name_default(&self, default: String) -> String {
        match &self.rename {
            Some(name) => name.clone(),
            None => default,
        }
    }
}

/// Parsed channel type
pub enum ChannelType {
    GuildText,
    Private,
    GuildVoice,
    Group,
    GuildCategory,
    GuildNews,
    GuildStore,
    GuildNewsThread,
    GuildPublicThread,
    GuildPrivateThread,
    GuildStageVoice,
}

impl ChannelType {
    /// Parse an [`AttrValue`] string as a [`ChannelType`]
    fn parse_attr(attr: &AttrValue) -> Result<Vec<Self>> {
        let span = attr.span();
        let val = attr.parse_string()?;

        val.split_whitespace()
            .map(|value| ChannelType::parse(value, span))
            .collect()
    }

    /// Parse a single string as a [`ChannelType`]
    fn parse(value: &str, span: Span) -> Result<Self> {
        match value {
            "guild_text" => Ok(Self::GuildText),
            "private" => Ok(Self::Private),
            "guild_voice" => Ok(Self::GuildVoice),
            "group" => Ok(Self::Group),
            "guild_category" => Ok(Self::GuildCategory),
            "guild_news" => Ok(Self::GuildNews),
            "guild_store" => Ok(Self::GuildStore),
            "guild_news_thread" => Ok(Self::GuildNewsThread),
            "guild_public_thread" => Ok(Self::GuildPublicThread),
            "guild_private_thread" => Ok(Self::GuildPrivateThread),
            "guild_stage_voice" => Ok(Self::GuildStageVoice),
            invalid => Err(Error::new(
                span,
                format!("`{}` is not a valid channel type", invalid),
            )),
        }
    }
}

/// Parsed command option value
#[derive(Clone, Copy)]
pub enum CommandOptionValue {
    Integer(i64),
    Number(f64),
}

impl CommandOptionValue {
    /// Parse an [`AttrValue`] as a [`CommandOptionValue`]
    fn parse_attr(attr: &AttrValue) -> Result<Self> {
        match attr.inner() {
            Lit::Int(inner) => Ok(Self::Integer(inner.base10_parse()?)),
            Lit::Float(inner) => Ok(Self::Number(inner.base10_parse()?)),
            _ => Err(Error::new(
                attr.span(),
                "Invalid attribute type, expected integer or float",
            )),
        }
    }
}

/// Convert a [`ChannelType`] into a [`TokenStream`]
pub fn channel_type(kind: &ChannelType) -> TokenStream {
    match kind {
        ChannelType::GuildText => quote!(::twilight_model::channel::ChannelType::GuildText),
        ChannelType::Private => quote!(::twilight_model::channel::ChannelType::Private),
        ChannelType::GuildVoice => quote!(::twilight_model::channel::ChannelType::GuildVoice),
        ChannelType::Group => quote!(::twilight_model::channel::ChannelType::Group),
        ChannelType::GuildCategory => quote!(::twilight_model::channel::ChannelType::GuildCategory),
        ChannelType::GuildNews => quote!(::twilight_model::channel::ChannelType::GuildNews),
        ChannelType::GuildStore => quote!(::twilight_model::channel::ChannelType::GuildStore),
        ChannelType::GuildNewsThread => {
            quote!(::twilight_model::channel::ChannelType::GuildNewsThread)
        }
        ChannelType::GuildPublicThread => {
            quote!(::twilight_model::channel::ChannelType::GuildPublicThread)
        }
        ChannelType::GuildPrivateThread => {
            quote!(::twilight_model::channel::ChannelType::GuildPrivateThread)
        }
        ChannelType::GuildStageVoice => {
            quote!(::twilight_model::channel::ChannelType::GuildStageVoice)
        }
    }
}

/// Convert a [`Option<CommandOptionValue>`] into a [`TokenStream`]
pub fn command_option_value(value: Option<CommandOptionValue>) -> TokenStream {
    match value {
        None => quote!(None),
        Some(CommandOptionValue::Integer(inner)) => {
            quote!(Some(::twilight_model::application::command::CommandOptionValue::Integer(#inner)))
        }
        Some(CommandOptionValue::Number(inner)) => quote! {
            Some(::twilight_model::application::command::CommandOptionValue::Number(
                ::twilight_model::application::command::Number(#inner)
            ))
        },
    }
}
