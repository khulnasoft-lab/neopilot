use indexmap::IndexMap;
use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColor {
    pub name: Option<String>,
    pub scope: Option<VsCodeTokenScope>,
    pub settings: VsCodeTokenColorSettings,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColorSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum NeopilotSyntaxToken {
    Attribute,
    Boolean,
    Comment,
    CommentDoc,
    Constant,
    Constructor,
    Embedded,
    Emphasis,
    EmphasisStrong,
    Enum,
    Function,
    Hint,
    Keyword,
    Label,
    LinkText,
    LinkUri,
    Number,
    Operator,
    Predictive,
    Preproc,
    Primary,
    Property,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationListMarker,
    PunctuationSpecial,
    String,
    StringEscape,
    StringRegex,
    StringSpecial,
    StringSpecialSymbol,
    Tag,
    TextLiteral,
    Title,
    Type,
    Variable,
    VariableSpecial,
    Variant,
}

impl std::fmt::Display for NeopilotSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NeopilotSyntaxToken::Attribute => "attribute",
                NeopilotSyntaxToken::Boolean => "boolean",
                NeopilotSyntaxToken::Comment => "comment",
                NeopilotSyntaxToken::CommentDoc => "comment.doc",
                NeopilotSyntaxToken::Constant => "constant",
                NeopilotSyntaxToken::Constructor => "constructor",
                NeopilotSyntaxToken::Embedded => "embedded",
                NeopilotSyntaxToken::Emphasis => "emphasis",
                NeopilotSyntaxToken::EmphasisStrong => "emphasis.strong",
                NeopilotSyntaxToken::Enum => "enum",
                NeopilotSyntaxToken::Function => "function",
                NeopilotSyntaxToken::Hint => "hint",
                NeopilotSyntaxToken::Keyword => "keyword",
                NeopilotSyntaxToken::Label => "label",
                NeopilotSyntaxToken::LinkText => "link_text",
                NeopilotSyntaxToken::LinkUri => "link_uri",
                NeopilotSyntaxToken::Number => "number",
                NeopilotSyntaxToken::Operator => "operator",
                NeopilotSyntaxToken::Predictive => "predictive",
                NeopilotSyntaxToken::Preproc => "preproc",
                NeopilotSyntaxToken::Primary => "primary",
                NeopilotSyntaxToken::Property => "property",
                NeopilotSyntaxToken::Punctuation => "punctuation",
                NeopilotSyntaxToken::PunctuationBracket => "punctuation.bracket",
                NeopilotSyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                NeopilotSyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                NeopilotSyntaxToken::PunctuationSpecial => "punctuation.special",
                NeopilotSyntaxToken::String => "string",
                NeopilotSyntaxToken::StringEscape => "string.escape",
                NeopilotSyntaxToken::StringRegex => "string.regex",
                NeopilotSyntaxToken::StringSpecial => "string.special",
                NeopilotSyntaxToken::StringSpecialSymbol => "string.special.symbol",
                NeopilotSyntaxToken::Tag => "tag",
                NeopilotSyntaxToken::TextLiteral => "text.literal",
                NeopilotSyntaxToken::Title => "title",
                NeopilotSyntaxToken::Type => "type",
                NeopilotSyntaxToken::Variable => "variable",
                NeopilotSyntaxToken::VariableSpecial => "variable.special",
                NeopilotSyntaxToken::Variant => "variant",
            }
        )
    }
}

impl NeopilotSyntaxToken {
    pub fn find_best_token_color_match<'a>(
        &self,
        token_colors: &'a [VsCodeTokenColor],
    ) -> Option<&'a VsCodeTokenColor> {
        let mut ranked_matches = IndexMap::new();

        for (ix, token_color) in token_colors.iter().enumerate() {
            if token_color.settings.foreground.is_none() {
                continue;
            }

            let Some(rank) = self.rank_match(token_color) else {
                continue;
            };

            if rank > 0 {
                ranked_matches.insert(ix, rank);
            }
        }

        ranked_matches
            .into_iter()
            .max_by_key(|(_, rank)| *rank)
            .map(|(ix, _)| &token_colors[ix])
    }

    fn rank_match(&self, token_color: &VsCodeTokenColor) -> Option<u32> {
        let candidate_scopes = match token_color.scope.as_ref()? {
            VsCodeTokenScope::One(scope) => vec![scope],
            VsCodeTokenScope::Many(scopes) => scopes.iter().collect(),
        }
        .iter()
        .flat_map(|scope| scope.split(',').map(|s| s.trim()))
        .collect::<Vec<_>>();

        let scopes_to_match = self.to_vscode();
        let number_of_scopes_to_match = scopes_to_match.len();

        let mut matches = 0;

        for (ix, scope) in scopes_to_match.into_iter().enumerate() {
            // Assign each entry a weight that is inversely proportional to its
            // position in the list.
            //
            // Entries towards the front are weighted higher than those towards the end.
            let weight = (number_of_scopes_to_match - ix) as u32;

            if candidate_scopes.contains(&scope) {
                matches += 1 + weight;
            }
        }

        Some(matches)
    }

    pub fn fallbacks(&self) -> &[Self] {
        match self {
            NeopilotSyntaxToken::CommentDoc => &[NeopilotSyntaxToken::Comment],
            NeopilotSyntaxToken::Number => &[NeopilotSyntaxToken::Constant],
            NeopilotSyntaxToken::VariableSpecial => &[NeopilotSyntaxToken::Variable],
            NeopilotSyntaxToken::PunctuationBracket
            | NeopilotSyntaxToken::PunctuationDelimiter
            | NeopilotSyntaxToken::PunctuationListMarker
            | NeopilotSyntaxToken::PunctuationSpecial => &[NeopilotSyntaxToken::Punctuation],
            NeopilotSyntaxToken::StringEscape
            | NeopilotSyntaxToken::StringRegex
            | NeopilotSyntaxToken::StringSpecial
            | NeopilotSyntaxToken::StringSpecialSymbol => &[NeopilotSyntaxToken::String],
            _ => &[],
        }
    }

    fn to_vscode(self) -> Vec<&'static str> {
        match self {
            NeopilotSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            NeopilotSyntaxToken::Boolean => vec!["constant.language"],
            NeopilotSyntaxToken::Comment => vec!["comment"],
            NeopilotSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            NeopilotSyntaxToken::Constant => {
                vec!["constant", "constant.language", "constant.character"]
            }
            NeopilotSyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
            }
            NeopilotSyntaxToken::Embedded => vec!["meta.embedded"],
            NeopilotSyntaxToken::Emphasis => vec!["markup.italic"],
            NeopilotSyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            NeopilotSyntaxToken::Enum => vec!["support.type.enum"],
            NeopilotSyntaxToken::Function => vec![
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            NeopilotSyntaxToken::Hint => vec![],
            NeopilotSyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
            NeopilotSyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            NeopilotSyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            NeopilotSyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            NeopilotSyntaxToken::Number => vec!["constant.numeric", "number"],
            NeopilotSyntaxToken::Operator => vec!["operator", "keyword.operator"],
            NeopilotSyntaxToken::Predictive => vec![],
            NeopilotSyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            NeopilotSyntaxToken::Primary => vec![],
            NeopilotSyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            NeopilotSyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.definition.tag",
            ],
            NeopilotSyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            NeopilotSyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            NeopilotSyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            NeopilotSyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            NeopilotSyntaxToken::String => vec!["string"],
            NeopilotSyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            NeopilotSyntaxToken::StringRegex => vec!["string.regex"],
            NeopilotSyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            NeopilotSyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            NeopilotSyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            NeopilotSyntaxToken::TextLiteral => vec!["text.literal", "string"],
            NeopilotSyntaxToken::Title => vec!["title", "entity.name"],
            NeopilotSyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            NeopilotSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            NeopilotSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            NeopilotSyntaxToken::Variant => vec!["variant"],
        }
    }
}
