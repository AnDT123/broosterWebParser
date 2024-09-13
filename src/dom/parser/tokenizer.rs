use crate::helper::stream::Stream;
use std::collections::VecDeque;
use std::cmp::max;

#[derive(Debug, Clone)]
pub enum Token {
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        force_quirks: bool,
    },
    StartTag {
        tag_name: String,
        self_closing: bool,
        attributes: Vec<(String, String)>,
    },
    EndTag {
        tag_name: String,
        self_closing: bool,
        attributes: Vec<(String, String)>,

    },
    Comment {
        data: String,
    },
    Character {
        data: char,
    },
    EndOfFile,
}
impl Token {
    pub fn attribute_exists(&self, name: &str) -> bool {
        match self {
            Token::StartTag { attributes, .. } | Token::EndTag { attributes, .. } => {
                attributes.iter().any(|(attr_name, _)| attr_name == name)
            },
            _ => false,
        }
    }
    pub fn add_attribute(&mut self, name: String, value: String) {
        match self {
            Token::StartTag { attributes, .. } | Token::EndTag { attributes, .. } => {
                if !attributes.iter().any(|(attr_name, _)| *attr_name == name) {
                    attributes.push((name, value));
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenizerState {
    Data,
    RCDATA,
    RAWTEXT,
    ScriptData,
    PLAINTEXT,
    TagOpen,
    EndTagOpen,
    TagName,
    RCDATALessThanSign,
    RCDATAEndTagOpen,
    RCDATAEndTagName,
    RAWTEXTLessThanSign,
    RAWTEXTEndTagOpen,
    RAWTEXTEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    DOCTYPE,
    BeforeDOCTYPEName,
    DOCTYPEName,
    AfterDOCTYPEName,
    AfterDOCTYPEPublicKeyword,
    BeforeDOCTYPEPublicIdentifier,
    DOCTYPEPublicIdentifierDoubleQuoted,
    DOCTYPEPublicIdentifierSingleQuoted,
    AfterDOCTYPEPublicIdentifier,
    BetweenDOCTYPEPublicAndSystemIdentifiers,
    AfterDOCTYPESystemKeyword,
    BeforeDOCTYPESystemIdentifier,
    DOCTYPESystemIdentifierDoubleQuoted,
    DOCTYPESystemIdentifierSingleQuoted,
    AfterDOCTYPESystemIdentifier,
    BogusDOCTYPE,
    CDATASection,
    CDATASectionBracket,
    CDATASectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}
pub struct Tokenizer<'a> {
    input_stream: Stream<'a, u8>,
    state: TokenizerState,
    ret_state: TokenizerState,
    current_tag_token: Option<Token>,
    tokens: Vec<Token>,
    temporary_buffer: String,
    last_start_tag_token: Option<Token> ,// this field is for end tag token validity check
    current_tag_name: String, //remember to clear after put into current_tag_token  
    current_tag_value: String, //same as above
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Tokenizer {
            input_stream: Stream::new(input),
            state: TokenizerState::Data,
            ret_state: TokenizerState::Data,
            current_tag_token: None,
            tokens: Vec::new(),
            temporary_buffer: String::new(),
            last_start_tag_token: None,
            current_tag_name: String::new(),
            current_tag_value: String::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.input_stream.is_eof() {
            match self.state {
                    TokenizerState::Data => self.handle_data_state(),
                    TokenizerState::RCDATA => self.handle_rcdata_state(),
                    TokenizerState::RAWTEXT => self.handle_rawtext_state(),
                    TokenizerState::ScriptData => self.handle_script_data_state(),
                    TokenizerState::PLAINTEXT => self.handle_plaintext_state(),
                    TokenizerState::TagOpen => self.handle_tag_open_state(),
                    TokenizerState::EndTagOpen => self.handle_end_tag_open_state(),
                    TokenizerState::TagName => self.handle_tag_name_state(),
                    TokenizerState::RCDATALessThanSign => self.handle_rcdata_less_than_sign_state(),
                    TokenizerState::RCDATAEndTagOpen => self.handle_rcdata_end_tag_open_state(),
                    TokenizerState::RCDATAEndTagName => self.handle_rcdata_end_tag_name_state(),
                    TokenizerState::RAWTEXTLessThanSign => self.handle_rawtext_less_than_sign_state(),
                    TokenizerState::RAWTEXTEndTagOpen => self.handle_rawtext_end_tag_open_state(),
                    TokenizerState::RAWTEXTEndTagName => self.handle_rawtext_end_tag_name_state(),
                    TokenizerState::ScriptDataLessThanSign => self.handle_script_data_less_than_sign_state(),
                    TokenizerState::ScriptDataEndTagOpen => self.handle_script_data_end_tag_open_state(),
                    TokenizerState::ScriptDataEndTagName => self.handle_script_data_end_tag_name_state(),
                    TokenizerState::ScriptDataEscapeStart => self.handle_script_data_escape_start_state(),
                    TokenizerState::ScriptDataEscapeStartDash => self.handle_script_data_escape_start_dash_state(),
                    TokenizerState::ScriptDataEscaped => self.handle_script_data_escaped_state(),
                    TokenizerState::ScriptDataEscapedDash => self.handle_script_data_escaped_dash_state(),
                    TokenizerState::ScriptDataEscapedDashDash => self.handle_script_data_escaped_dash_dash_state(),
                    TokenizerState::ScriptDataEscapedLessThanSign => self.handle_script_data_escaped_less_than_sign_state(),
                    TokenizerState::ScriptDataEscapedEndTagOpen => self.handle_script_data_escaped_end_tag_open_state(),
                    TokenizerState::ScriptDataEscapedEndTagName => self.handle_script_data_escaped_end_tag_name_state(),
                    TokenizerState::ScriptDataDoubleEscapeStart => self.handle_script_data_double_escape_start_state(),
                    TokenizerState::ScriptDataDoubleEscaped => self.handle_script_data_double_escaped_state(),
                    TokenizerState::ScriptDataDoubleEscapedDash => self.handle_script_data_double_escaped_dash_state(),
                    TokenizerState::ScriptDataDoubleEscapedDashDash => self.handle_script_data_double_escaped_dash_dash_state(),
                    TokenizerState::ScriptDataDoubleEscapedLessThanSign => self.handle_script_data_double_escaped_less_than_sign_state(),
                    TokenizerState::ScriptDataDoubleEscapeEnd => self.handle_script_data_double_escape_end_state(),
                    TokenizerState::BeforeAttributeName => self.handle_before_attribute_name_state(),
                    TokenizerState::AttributeName => self.handle_attribute_name_state(),
                    TokenizerState::AfterAttributeName => self.handle_after_attribute_name_state(),
                    TokenizerState::BeforeAttributeValue => self.handle_before_attribute_value_state(),
                    TokenizerState::AttributeValueDoubleQuoted => self.handle_attribute_value_double_quoted_state(),
                    TokenizerState::AttributeValueSingleQuoted => self.handle_attribute_value_single_quoted_state(),
                    TokenizerState::AttributeValueUnquoted => self.handle_attribute_value_unquoted_state(),
                    TokenizerState::AfterAttributeValueQuoted => self.handle_after_attribute_value_quoted_state(),
                    TokenizerState::SelfClosingStartTag => self.handle_self_closing_start_tag_state(),
                    TokenizerState::BogusComment => self.handle_bogus_comment_state(),
                    TokenizerState::MarkupDeclarationOpen => self.handle_markup_declaration_open_state(),
                    TokenizerState::CommentStart => self.handle_comment_start_state(),
                    TokenizerState::CommentStartDash => self.handle_comment_start_dash_state(),
                    TokenizerState::Comment => self.handle_comment_state(),
                    TokenizerState::CommentLessThanSign => self.handle_comment_less_than_sign_state(),
                    TokenizerState::CommentLessThanSignBang => self.handle_comment_less_than_sign_bang_state(),
                    TokenizerState::CommentLessThanSignBangDash => self.handle_comment_less_than_sign_bang_dash_state(),
                    TokenizerState::CommentLessThanSignBangDashDash => self.handle_comment_less_than_sign_bang_dash_dash_state(),
                    TokenizerState::CommentEndDash => self.handle_comment_end_dash_state(),
                    TokenizerState::CommentEnd => self.handle_comment_end_state(),
                    TokenizerState::CommentEndBang => self.handle_comment_end_bang_state(),
                    TokenizerState::DOCTYPE => self.handle_doctype_state(),
                    TokenizerState::BeforeDOCTYPEName => self.handle_before_doctype_name_state(),
                    TokenizerState::DOCTYPEName => self.handle_doctype_name_state(),
                    TokenizerState::AfterDOCTYPEName => self.handle_after_doctype_name_state(),
                    TokenizerState::AfterDOCTYPEPublicKeyword => self.handle_after_doctype_public_keyword_state(),
                    TokenizerState::BeforeDOCTYPEPublicIdentifier => self.handle_before_doctype_public_identifier_state(),
                    TokenizerState::DOCTYPEPublicIdentifierDoubleQuoted => self.handle_doctype_public_identifier_double_quoted_state(),
                    TokenizerState::DOCTYPEPublicIdentifierSingleQuoted => self.handle_doctype_public_identifier_single_quoted_state(),
                    TokenizerState::AfterDOCTYPEPublicIdentifier => self.handle_after_doctype_public_identifier_state(),
                    TokenizerState::BetweenDOCTYPEPublicAndSystemIdentifiers => self.handle_between_doctype_public_and_system_identifiers_state(),
                    TokenizerState::AfterDOCTYPESystemKeyword => self.handle_after_doctype_system_keyword_state(),
                    TokenizerState::BeforeDOCTYPESystemIdentifier => self.handle_before_doctype_system_identifier_state(),
                    TokenizerState::DOCTYPESystemIdentifierDoubleQuoted => self.handle_doctype_system_identifier_double_quoted_state(),
                    TokenizerState::DOCTYPESystemIdentifierSingleQuoted => self.handle_doctype_system_identifier_single_quoted_state(),
                    TokenizerState::AfterDOCTYPESystemIdentifier => self.handle_after_doctype_system_identifier_state(),
                    TokenizerState::BogusDOCTYPE => self.handle_bogus_doctype_state(),
                    TokenizerState::CDATASection => self.handle_cdata_section_state(),
                    TokenizerState::CDATASectionBracket => self.handle_cdata_section_bracket_state(),
                    TokenizerState::CDATASectionEnd => self.handle_cdata_section_end_state(),
                    TokenizerState::CharacterReference => self.handle_character_reference_state(),
                    TokenizerState::NamedCharacterReference => self.handle_named_character_reference_state(),
                    TokenizerState::AmbiguousAmpersand => self.handle_ambiguous_ampersand_state(),
                    TokenizerState::NumericCharacterReference => self.handle_numeric_character_reference_state(),
                    TokenizerState::HexadecimalCharacterReferenceStart => self.handle_hexadecimal_character_reference_start_state(),
                    TokenizerState::DecimalCharacterReferenceStart => self.handle_decimal_character_reference_start_state(),
                    TokenizerState::HexadecimalCharacterReference => self.handle_hexadecimal_character_reference_state(),
                    TokenizerState::DecimalCharacterReference => self.handle_decimal_character_reference_state(),
                    TokenizerState::NumericCharacterReferenceEnd => self.handle_numeric_character_reference_end_state(),
                }
            }
        
    }
    
    fn handle_data_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'&') => {
                self.ret_state = TokenizerState::Data;
                self.state = TokenizerState::CharacterReference;
            }
            Some(b'<') => self.state = TokenizerState::TagOpen, 
            Some(b'\0') => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character{data: next_char.unwrap() as char});
            }
            None => self.emit_token(Token::EndOfFile),
            Some(ch) => self.emit_token(Token::Character{data: ch as char}), 
        }
    }

    fn handle_rcdata_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'&') => {
                self.ret_state = TokenizerState::RCDATA;
                self.state = TokenizerState::CharacterReference;
            }
            Some(b'<') => self.state = TokenizerState::RCDATALessThanSign, 
            Some(b'\0') => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character{data: '\u{FFFD}'}); //REPLACEMENT CHARACTER character token.
            }
            None => self.emit_token(Token::EndOfFile), 
            Some(ch) => self.emit_token(Token::Character{data: ch as char}),
        }
    }

    fn handle_rawtext_state(&mut self) {
       let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'<') => self.state = TokenizerState::RAWTEXTLessThanSign,
            Some(b'\0') => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character{data: '\u{FFFD}'});
            }
            None => self.emit_token(Token::EndOfFile),
            Some(ch) => self.emit_token(Token::Character{data: ch as char}),
        }
    }

    fn handle_script_data_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'<') => self.state = TokenizerState::ScriptDataLessThanSign,
            Some(b'\0') => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character{data: '\u{FFFD}'});
            }
            None => self.emit_token(Token::EndOfFile),
            Some(ch) => self.emit_token(Token::Character{data: ch as char}),
        }
    }

    fn handle_plaintext_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'\0') => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character{data: '\u{FFFD}'});
            }
            None => self.emit_token(Token::EndOfFile),
            Some(ch) => self.emit_token(Token::Character{data: ch as char}),
        }
    }

    fn handle_tag_open_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'!') => self.state = TokenizerState::MarkupDeclarationOpen,
            Some(b'/') => self.state = TokenizerState::EndTagOpen,
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.current_tag_token = Some(Token::StartTag {
                    tag_name: String::new(),
                    self_closing: false,
                    attributes: Vec::new(),
                });
                self.state = TokenizerState::TagName;
                self.reconsume_char();
            }
            Some(b'?') => {
                self.emit_parse_error("unexpected-question-mark-instead-of-tag-name");
                self.emit_token(Token::Comment{data:String::new()});
                self.state = TokenizerState::BogusComment;
                self.reconsume_char();
            }
            None => {
                self.emit_parse_error(" eof-before-tag-name");
                self.emit_token(Token::Character{data: '<'});
                self.emit_token(Token::EndOfFile);
            }
            Some(_) => {
                self.emit_parse_error("invalid-first-character-of-tag-name");
                self.emit_token(Token::Character{data: '<'});
                self.state = TokenizerState::Data;
                self.reconsume_char();
            }
        }
    }

    fn handle_end_tag_open_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.current_tag_token = Some(Token::EndTag {
                    tag_name: String::new(),
                    self_closing: false,
                    attributes: Vec::new(),
                });
                self.state = TokenizerState::TagName;
                self.reconsume_char();
            }
            Some(b'>') => {
                self.emit_parse_error("missing-end-tag-name");
                self.state = TokenizerState::Data;
            }
            None => {
                self.emit_parse_error("eof-before-tag-name");
                self.emit_token(Token::Character{data: '<'});
                self.emit_token(Token::Character{data: '/'});
                self.emit_token(Token::EndOfFile);
            }
            Some(_) => {
                self.emit_parse_error("invalid-first-character-of-tag-name");
                self.emit_token(Token::Comment{data:String::new()});
                self.state = TokenizerState::BogusComment;
                self.reconsume_char();
            }
        }
    }

    fn handle_tag_name_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
                self.state = TokenizerState::BeforeAttributeName;
            }
            Some(b'/') => {
                self.state = TokenizerState::SelfClosingStartTag;
            }
            Some(b'>') => {
                self.state = TokenizerState::Data;
                if let Some(token) = self.current_tag_token.clone() {
                    self.emit_token(token);
                }
            }
            Some(ch) if ch.is_ascii_uppercase() => {
                if let Some(Token::StartTag { tag_name, .. }) = self.current_tag_token.as_mut() {
                    tag_name.push((ch + 0x20) as char);
                }
            }
            Some(b'\0') => {
                self.emit_parse_error("unexpected-null-character");
                if let Some(Token::StartTag { tag_name, .. }) = self.current_tag_token.as_mut() {
                    tag_name.push('\u{FFFD}');
                }
            }
            None => {
                self.emit_parse_error("Parse error: EOF in tag");
                self.emit_token(Token::EndOfFile);
            }
            Some(ch) => {
                if let Some(Token::StartTag { tag_name, .. }) = self.current_tag_token.as_mut() {
                    tag_name.push(ch as char);
                }
            }
        }
    }

    fn handle_rcdata_less_than_sign_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'/') => {
                self.temporary_buffer = String::new();
                self.state = TokenizerState::RCDATAEndTagOpen;
            }
            _ => {
                self.emit_token(Token::Character{data: '<'});
                self.state = TokenizerState::RCDATA; 
                self.reconsume_char();
            }
        }
    }
    
    fn handle_rcdata_end_tag_open_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.current_tag_token = Some(Token::EndTag {
                    tag_name: String::new(),
                    self_closing: false,       
                    attributes: Vec::new(),    
                
                });
                self.state = TokenizerState::RCDATAEndTagName;
                self.reconsume_char();
            }
            _ => {
                self.emit_token(Token::Character{data: '<'});
                self.emit_token(Token::Character{data: '/'});
                self.state = TokenizerState::RCDATA;
                self.reconsume_char();
            }
        }
    }

    fn handle_rcdata_end_tag_name_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
            if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::BeforeAttributeName;
                } else {
                    self.handle_rcdata_end_tag_name_state_anything_else();
                }
            }

            Some(b'/') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::SelfClosingStartTag;
                } else {
                    self.handle_rcdata_end_tag_name_state_anything_else();
                }
            }

            Some(b'>') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::Data;
                    if let Some(token) = self.current_tag_token.clone() {
                        self.emit_token(token);
                    }
                } else {
                    self.handle_rcdata_end_tag_name_state_anything_else();
                }
            }

            Some(ch) if ch.is_ascii_uppercase() => {
                if let Some(Token::EndTag { ref mut tag_name,.. }) = self.current_tag_token.as_mut() {
                    tag_name.push((ch + 0x20) as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            Some(ch) if ch.is_ascii_lowercase() => {
                if let Some(Token::EndTag { ref mut tag_name,.. }) = self.current_tag_token.as_mut() {
                    tag_name.push(ch as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            _ => {
                self.handle_rcdata_end_tag_name_state_anything_else();
            }
        }
    }

    fn handle_rcdata_end_tag_name_state_anything_else(&mut self) {

        self.emit_token(Token::Character { data: '<' });
        self.emit_token(Token::Character { data: '/' });
        
        let chars: Vec<char> = self.temporary_buffer.chars().collect();
        for ch in chars {
            self.emit_token(Token::Character { data: ch });
        }
        
        self.temporary_buffer.clear();

        self.state = TokenizerState::RCDATA;
        self.reconsume_char();
    }


    fn is_appropriate_end_tag_token(&self) -> bool {
        match (&self.current_tag_token, &self.last_start_tag_token) {
            (Some(Token::EndTag { tag_name: end_tag_name,.. }), Some(Token::StartTag { tag_name: start_tag_name, .. })) => {
                end_tag_name == start_tag_name
            },
            _ => false,
        }
    }


    fn handle_rawtext_less_than_sign_state(&mut self) {
        let next_char = self.consume_next_input_char();
        match next_char {
            Some(b'/') => {
                self.temporary_buffer.clear();
                self.state = TokenizerState::RAWTEXTEndTagOpen;
            }
            _ => {
                self.emit_token(Token::Character { data: '<' });
                self.state = TokenizerState::RAWTEXT;
                self.reconsume_char();
            }
        }
    }

    fn handle_rawtext_end_tag_open_state(&mut self) {
        let next_char = self.consume_next_input_char();
        match next_char {
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.current_tag_token = Some(Token::EndTag { tag_name: String::new(), self_closing: false, attributes: Vec::new(),});
                self.state = TokenizerState::RAWTEXTEndTagName;
                self.reconsume_char();
            }
            _ => {
                self.emit_token(Token::Character { data: '<' });
                self.emit_token(Token::Character { data: '/' });
                self.state = TokenizerState::RAWTEXT;
                self.reconsume_char();
            }
        }
    }

    fn handle_rawtext_end_tag_name_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
            if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::BeforeAttributeName;
                } else {
                    self.handle_rawtext_end_tag_name_state_anything_else();
                }
            }

            Some(b'/') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::SelfClosingStartTag;
                } else {
                    self.handle_rawtext_end_tag_name_state_anything_else();
                }
            }

            Some(b'>') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::Data;
                    if let Some(token) = self.current_tag_token.clone() {
                        self.emit_token(token);
                    }
                } else {
                    self.handle_rawtext_end_tag_name_state_anything_else();
                }
            }

            Some(ch) if ch.is_ascii_uppercase() => {
                if let Some(Token::EndTag { ref mut tag_name,..}) = self.current_tag_token.as_mut() {
                    tag_name.push((ch + 0x20) as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            Some(ch) if ch.is_ascii_lowercase() => {
                if let Some(Token::EndTag { ref mut tag_name,..}) = self.current_tag_token.as_mut() {
                    tag_name.push(ch as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            _ => {
                self.handle_rawtext_end_tag_name_state_anything_else();
            }
    }

    }
    fn handle_rawtext_end_tag_name_state_anything_else(&mut self) {

        self.emit_token(Token::Character { data: '<' });
        self.emit_token(Token::Character { data: '/' });
        
        let chars: Vec<char> = self.temporary_buffer.chars().collect();
        for ch in chars {
            self.emit_token(Token::Character { data: ch });
        }
        
        self.temporary_buffer.clear();

        self.state = TokenizerState::RAWTEXT;
        self.reconsume_char();
    }

    fn handle_script_data_less_than_sign_state(&mut self) {
        let next_char = self.consume_next_input_char();
        match next_char {
            Some(b'/') => {
                self.temporary_buffer.clear();
                self.state = TokenizerState::ScriptDataEndTagOpen;
            }
            Some(b'!') => {
                self.state = TokenizerState::ScriptDataEscapeStart;
                self.emit_token(Token::Character { data: '<' });
                self.emit_token(Token::Character { data: '!' });
            }
            _ => {
                self.emit_token(Token::Character { data: '<' });
                self.reconsume_char();
            }
        }
    }


    fn handle_script_data_end_tag_open_state(&mut self) {
        let next_char = self.consume_next_input_char();
        match next_char {
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.current_tag_token = Some(Token::EndTag { tag_name: String::new() ,self_closing: false, attributes: Vec::new()});
                self.state = TokenizerState::ScriptDataEndTagName;
                self.reconsume_char();
            }
            _ => {
                self.emit_token(Token::Character { data: '<' });
                self.emit_token(Token::Character { data: '/' });
                self.state = TokenizerState::ScriptData;
                self.reconsume_char();
            }
        }
    }
    fn handle_script_data_end_tag_name_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
            if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::BeforeAttributeName;
                } else {
                    self.handle_script_end_tag_name_state_anything_else();
                }
            }

            Some(b'/') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::SelfClosingStartTag;
                } else {
                    self.handle_script_end_tag_name_state_anything_else();
                }
            }

            Some(b'>') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::Data;
                    if let Some(token) = self.current_tag_token.clone() {
                        self.emit_token(token);
                    }
                } else {
                    self.handle_script_end_tag_name_state_anything_else();
                }
            }

            Some(ch) if ch.is_ascii_uppercase() => {
                if let Some(Token::EndTag { ref mut tag_name,..}) = self.current_tag_token.as_mut() {
                    tag_name.push((ch + 0x20) as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            Some(ch) if ch.is_ascii_lowercase() => {
                if let Some(Token::EndTag { ref mut tag_name,..}) = self.current_tag_token.as_mut() {
                    tag_name.push(ch as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            _ => {
                self.handle_script_end_tag_name_state_anything_else();
            }
        }
    }
    fn handle_script_end_tag_name_state_anything_else(&mut self) {

        self.emit_token(Token::Character { data: '<' });
        self.emit_token(Token::Character { data: '/' });
        
        let chars: Vec<char> = self.temporary_buffer.chars().collect();
        for ch in chars {
            self.emit_token(Token::Character { data: ch });
        }
        
        self.temporary_buffer.clear();

        self.state = TokenizerState::ScriptData;
        self.reconsume_char();
    }

    fn handle_script_data_escape_start_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'-') => {
                self.state = TokenizerState::ScriptDataEscapeStartDash;
                self.emit_token(Token::Character { data: '-' });
            }
    
            _ => {
                self.state = TokenizerState::ScriptData;
                self.reconsume_char();
            }
        }
    }

    fn handle_script_data_escape_start_dash_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'-') => {
                self.state = TokenizerState::ScriptDataEscapedDashDash;
                self.emit_token(Token::Character { data: '-' });
            }

            _ => {
                self.state = TokenizerState::ScriptData;
                self.reconsume_char(); 
            }
        }
    }

    fn handle_script_data_escaped_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'-') => {
                self.state = TokenizerState::ScriptDataEscapedDash;
                self.emit_token(Token::Character { data: '-' });
            }
    
            Some(b'<') => {
                self.state = TokenizerState::ScriptDataEscapedLessThanSign;
            }
    
            Some(0x00) => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character { data: '\u{FFFD}' }); // Emit a replacement character (U+FFFD)
            }
    
            None => {
                self.emit_parse_error("eof-in-script-html-comment-like-text");
                self.emit_token(Token::EndOfFile);
            }
    
            Some(ch) => {
                self.emit_token(Token::Character { data: ch as char});
            }
        }
    }
    
    //13.2.5.21 Script data escaped dash state
    fn handle_script_data_escaped_dash_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'-') => {
                self.state = TokenizerState::ScriptDataEscapedDashDash;
                self.emit_token(Token::Character { data: '-' });
            }
    
            Some(b'<') => {
                self.state = TokenizerState::ScriptDataEscapedLessThanSign;
            }
    
            Some(0x00) => {
                self.emit_parse_error("unexpected-null-character");
                self.state = TokenizerState::ScriptDataEscaped;
                self.emit_token(Token::Character { data: '\u{FFFD}' });
            }
    
            // Handling EOF
            None => {
                self.emit_parse_error("eof-in-script-html-comment-like-text");
                self.emit_token(Token::EndOfFile);
            }
    
            Some(ch) => {
                self.state = TokenizerState::ScriptDataEscaped;
                self.emit_token(Token::Character { data: ch as char});
            }
        }
    }
    
    //13.2.5.22 Script data escaped dash dash state
    fn handle_script_data_escaped_dash_dash_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'-') => {
                self.emit_token(Token::Character { data: '-' });
            }
    
            Some(b'<') => {
                self.state = TokenizerState::ScriptDataEscapedLessThanSign;
            }
    
            Some(b'>') => {
                self.state = TokenizerState::ScriptData;
                self.emit_token(Token::Character { data: '>' });
            }
    
            Some(0x00) => {
                self.emit_parse_error("unexpected-null-character");
                self.state = TokenizerState::ScriptDataEscaped;
                self.emit_token(Token::Character { data: '\u{FFFD}' }); // Emit a replacement character (U+FFFD)
            }
    
            None => {
                self.emit_parse_error("eof-in-script-html-comment-like-text");
                self.emit_token(Token::EndOfFile);
            }
    
            Some(ch) => {
                self.state = TokenizerState::ScriptDataEscaped;
                self.emit_token(Token::Character { data: ch as char});
            }
        }
    }
    
    //13.2.5.23 Script data escaped less-than sign state
    fn handle_script_data_escaped_less_than_sign_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'/') => {
                self.temporary_buffer.clear();
                self.state = TokenizerState::ScriptDataEscapedEndTagOpen;
            }
    
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.temporary_buffer.clear();
                self.emit_token(Token::Character { data: '<' });
                self.state = TokenizerState::ScriptDataDoubleEscapeStart;
                self.reconsume_char();
            }
    
            _ => {
                self.emit_token(Token::Character { data: '<' });
                self.state = TokenizerState::ScriptDataEscaped;
                self.reconsume_char(); 
            }
        }
    }

    //13.2.5.24 Script data escaped end tag open state
    fn handle_script_data_escaped_end_tag_open_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(ch) if ch.is_ascii_alphabetic() => {
                self.current_tag_token = Some(Token::EndTag { tag_name: String::new() , self_closing: false, attributes: Vec::new()});
                self.state = TokenizerState::ScriptDataEscapedEndTagName;
                self.reconsume_char();
            }
    
            _ => {
                self.emit_token(Token::Character { data: '<' });
                self.emit_token(Token::Character { data: '/' });
                self.state = TokenizerState::ScriptDataEscaped;
                self.reconsume_char();
            }
        }
    }
    
    //13.2.5.25 Script data escaped end tag name state
    fn handle_script_data_escaped_end_tag_name_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::BeforeAttributeName;
                } else {
                    self.handle_script_data_escaped_end_tag_name_state_anything_else();
                }
            }
    
            Some(b'/') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::SelfClosingStartTag;
                } else {
                    self.handle_script_data_escaped_end_tag_name_state_anything_else();
                }
            }
    
            Some(b'>') => {
                if self.is_appropriate_end_tag_token() {
                    self.state = TokenizerState::Data;
                    if let Some(token) = self.current_tag_token.clone() {
                        self.emit_token(token);
                    }
                } else {
                    self.handle_script_data_escaped_end_tag_name_state_anything_else();
                }
            }
    
            Some(ch) if ch.is_ascii_uppercase() => {
                if let Some(Token::EndTag { ref mut tag_name,..}) = self.current_tag_token.as_mut() {
                    tag_name.push((ch + 0x20) as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }

            Some(ch) if ch.is_ascii_lowercase() => {
                if let Some(Token::EndTag { ref mut tag_name,..}) = self.current_tag_token.as_mut() {
                    tag_name.push(ch as char); 
                }
                self.temporary_buffer.push(ch as char); 
            }
    
            _ => {
                self.handle_script_data_escaped_end_tag_name_state_anything_else();
            }
        }
    }
    
    fn handle_script_data_escaped_end_tag_name_state_anything_else(&mut self){
        self.emit_token(Token::Character { data: '<' });
        self.emit_token(Token::Character { data: '/' });
        
        let chars: Vec<char> = self.temporary_buffer.chars().collect();
        for ch in chars {
            self.emit_token(Token::Character { data: ch });
        }
        
        self.temporary_buffer.clear();

        self.state = TokenizerState::ScriptDataEscaped;
        self.reconsume_char();
    }

    // 13.2.5.26 Script data double escape start state
    fn handle_script_data_double_escape_start_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') | Some(b'/') | Some(b'>') => {
                if self.temporary_buffer == "script" {
                    self.state = TokenizerState::ScriptDataDoubleEscaped;
                } else {
                    self.state = TokenizerState::ScriptDataEscaped;
                }
                self.emit_token(Token::Character { data: next_char.unwrap() as char });
            }

            Some(ch) if ch.is_ascii_uppercase() => {
                self.temporary_buffer.push((ch + 0x20) as char);
                self.emit_token(Token::Character { data: ch as char });
            }

            Some(ch) if ch.is_ascii_lowercase() => {
                self.temporary_buffer.push(ch as char);
                self.emit_token(Token::Character { data: ch as char });
            }

            _ => {
                self.state = TokenizerState::ScriptDataEscaped;
                self.reconsume_char();
            }
        }
    }

    // 13.2.5.27 Script data double escaped state
    fn handle_script_data_double_escaped_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'-') => {
                self.state = TokenizerState::ScriptDataDoubleEscapedDash;
                self.emit_token(Token::Character { data: '-' });
            }

            Some(b'<') => {
                self.state = TokenizerState::ScriptDataDoubleEscapedLessThanSign;
                self.emit_token(Token::Character { data: '<' });
            }

            Some(0x00) => {
                self.emit_parse_error("unexpected-null-character");
                self.emit_token(Token::Character { data: '\u{FFFD}' });
            }

            None => {
                self.emit_parse_error("eof-in-script-html-comment-like-text");
                self.emit_token(Token::EndOfFile);
            }

            Some(ch) => {
                self.emit_token(Token::Character { data: ch as char });
            }
        }
    }

    // 13.2.5.28 Script data double escaped dash state
    fn handle_script_data_double_escaped_dash_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'-') => {
                self.state = TokenizerState::ScriptDataDoubleEscapedDashDash;
                self.emit_token(Token::Character { data: '-' });
            }

            Some(b'<') => {
                self.state = TokenizerState::ScriptDataDoubleEscapedLessThanSign;
                self.emit_token(Token::Character { data: '<' });
            }

            Some(0x00) => {
                self.emit_parse_error("unexpected-null-character");
                self.state = TokenizerState::ScriptDataDoubleEscaped;
                self.emit_token(Token::Character { data: '\u{FFFD}' });
            }

            None => {
                self.emit_parse_error("eof-in-script-html-comment-like-text");
                self.emit_token(Token::EndOfFile);
            }

            Some(ch) => {
                self.state = TokenizerState::ScriptDataDoubleEscaped;
                self.emit_token(Token::Character { data: ch as char });
            }
        }
    }

    // 13.2.5.29 Script data double escaped dash dash state
    fn handle_script_data_double_escaped_dash_dash_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'-') => {
                self.emit_token(Token::Character { data: '-' });
            }

            Some(b'<') => {
                self.state = TokenizerState::ScriptDataDoubleEscapedLessThanSign;
                self.emit_token(Token::Character { data: '<' });
            }

            Some(b'>') => {
                self.state = TokenizerState::ScriptData;
                self.emit_token(Token::Character { data: '>' });
            }

            Some(0x00) => {
                self.emit_parse_error("unexpected-null-character");
                self.state = TokenizerState::ScriptDataDoubleEscaped;
                self.emit_token(Token::Character { data: '\u{FFFD}' });
            }

            None => {
                self.emit_parse_error("eof-in-script-html-comment-like-text");
                self.emit_token(Token::EndOfFile);
            }

            Some(ch) => {
                self.state = TokenizerState::ScriptDataDoubleEscaped;
                self.emit_token(Token::Character { data: ch as char });
            }
        }
    }

    // 13.2.5.30 Script data double escaped less-than sign state
    fn handle_script_data_double_escaped_less_than_sign_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'/') => {
                self.temporary_buffer.clear();
                self.state = TokenizerState::ScriptDataDoubleEscapeEnd;
                self.emit_token(Token::Character { data: '/' });
            }

            _ => {
                self.state = TokenizerState::ScriptDataDoubleEscaped;
                self.reconsume_char();
            }
        }
    }

    // 13.2.5.31 Script data double escape end state
    fn handle_script_data_double_escape_end_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') | Some(b'/') | Some(b'>') => {
                if self.temporary_buffer == "script" {
                    self.state = TokenizerState::ScriptDataEscaped;
                } else {
                    self.state = TokenizerState::ScriptDataDoubleEscaped;
                }
                self.emit_token(Token::Character { data: next_char.unwrap() as char });
            }

            Some(ch) if ch.is_ascii_uppercase() => {
                self.temporary_buffer.push((ch + 0x20) as char);
                self.emit_token(Token::Character { data: ch as char });
            }

            Some(ch) if ch.is_ascii_lowercase() => {
                self.temporary_buffer.push(ch as char);
                self.emit_token(Token::Character { data: ch as char });
            }

            _ => {
                self.state = TokenizerState::ScriptDataDoubleEscaped;
                self.reconsume_char();
            }
        }
    }

    fn handle_before_attribute_name_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
            }

            Some(b'/') | Some(b'>') | None => {
                self.state = TokenizerState::AfterAttributeName;
                self.reconsume_char();
            }

            Some(b'=') => {
                self.emit_parse_error("unexpected-equals-sign-before-attribute-name");
                let name= "=".to_string(); //need to check attribute name duplication before putting in the current_tag_token
                self.current_tag_value.clear();
                self.state = TokenizerState::AttributeName;
            }

            Some(_) => {
                self.current_tag_name.clear();
                self.current_tag_value.clear();
                self.state = TokenizerState::AttributeName;
                self.reconsume_char();
            }
        }
    }

    //13.2.5.33 Attribute name state
    fn handle_attribute_name_state(&mut self) {
        let next_char = self.consume_next_input_char();

        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') |
            Some(b'/') | Some(b'>') | None => {
                self.state = TokenizerState::AfterAttributeName;
                self.reconsume_char();
            }

            Some(b'=') => {
                self.state = TokenizerState::BeforeAttributeValue;
            }

            Some(c) if c.is_ascii_uppercase() => {
                self.current_tag_name.push((c + 0x20) as char);
            }

            Some(b'\x00') => {
                self.emit_parse_error("unexpected-null-character");
                self.current_tag_name.push('\u{FFFD}' as char);
            }

            Some(b'"') | Some(b'\'') | Some(b'<') => {
                self.emit_parse_error("unexpected-character-in-attribute-name");
                self.current_tag_name.push(next_char.unwrap() as char);
            }

            Some(_) => {
                self.current_tag_name.push(next_char.unwrap() as char);
            }
        }
    }

    //13.2.5.34 After attribute name state
    fn handle_after_attribute_name_state(&mut self) {
        let next_char = self.consume_next_input_char();
    
        match next_char {
            Some(b'\t') | Some(b'\n') | Some(b'\x0C') | Some(b' ') => {
            }
    
            Some(b'/') => {
                //no value next so add name to current_tag_token
                self.add_attribute_to_current_tag_token();
                
                self.state = TokenizerState::SelfClosingStartTag;
            }
    
            Some(b'=') => {
                // there's a value after name
                self.state = TokenizerState::BeforeAttributeValue;
            }
    
            Some(b'>') => {
                //no value next so add name to current_tag_token
                self.add_attribute_to_current_tag_token();

                self.state = TokenizerState::Data;
                self.emit_current_tag_token();
            }
    
            None => {
                //no value next so add name to current_tag_token
                self.add_attribute_to_current_tag_token();

                self.emit_parse_error("eof-in-tag");
                self.emit_token(Token::EndOfFile);
            }
    
            Some(_) => {
                //no value next so add name to current_tag_token
                self.add_attribute_to_current_tag_token();

                self.state = TokenizerState::AttributeName;
                self.reconsume_char();
            }
        }
    }
    

    fn handle_before_attribute_value_state(&mut self) {
        // Implementation for Before attribute value state
    }

    fn handle_attribute_value_double_quoted_state(&mut self) {
        // Implementation for Attribute value (double-quoted) state
    }

    fn handle_attribute_value_single_quoted_state(&mut self) {
        // Implementation for Attribute value (single-quoted) state
    }

    fn handle_attribute_value_unquoted_state(&mut self) {
        // Implementation for Attribute value (unquoted) state
    }

    fn handle_after_attribute_value_quoted_state(&mut self) {
        // Implementation for After attribute value (quoted) state
    }

    fn handle_self_closing_start_tag_state(&mut self) {
        // Implementation for Self-closing start tag state
    }

    fn handle_bogus_comment_state(&mut self) {
        // Implementation for Bogus comment state
    }

    fn handle_markup_declaration_open_state(&mut self) {
        // Implementation for Markup declaration open state
    }

    fn handle_comment_start_state(&mut self) {
        // Implementation for Comment start state
    }

    fn handle_comment_start_dash_state(&mut self) {
        // Implementation for Comment start dash state
    }

    fn handle_comment_state(&mut self) {
        // Implementation for Comment state
    }

    fn handle_comment_less_than_sign_state(&mut self) {
        // Implementation for Comment less-than sign state
    }

    fn handle_comment_less_than_sign_bang_state(&mut self) {
        // Implementation for Comment less-than sign bang state
    }

    fn handle_comment_less_than_sign_bang_dash_state(&mut self) {
        // Implementation for Comment less-than sign bang dash state
    }

    fn handle_comment_less_than_sign_bang_dash_dash_state(&mut self) {
        // Implementation for Comment less-than sign bang dash dash state
    }

    fn handle_comment_end_dash_state(&mut self) {
        // Implementation for Comment end dash state
    }

    fn handle_comment_end_state(&mut self) {
        // Implementation for Comment end state
    }

    fn handle_comment_end_bang_state(&mut self) {
        // Implementation for Comment end bang state
    }

    fn handle_doctype_state(&mut self) {
        // Implementation for DOCTYPE state
    }

    fn handle_before_doctype_name_state(&mut self) {
        // Implementation for Before DOCTYPE name state
    }

    fn handle_doctype_name_state(&mut self) {
        // Implementation for DOCTYPE name state
    }

    fn handle_after_doctype_name_state(&mut self) {
        // Implementation for After DOCTYPE name state
    }

    fn handle_after_doctype_public_keyword_state(&mut self) {
        // Implementation for After DOCTYPE public keyword state
    }

    fn handle_before_doctype_public_identifier_state(&mut self) {
        // Implementation for Before DOCTYPE public identifier state
    }

    fn handle_doctype_public_identifier_double_quoted_state(&mut self) {
        // Implementation for DOCTYPE public identifier (double-quoted) state
    }

    fn handle_doctype_public_identifier_single_quoted_state(&mut self) {
        // Implementation for DOCTYPE public identifier (single-quoted) state
    }

    fn handle_after_doctype_public_identifier_state(&mut self) {
        // Implementation for After DOCTYPE public identifier state
    }

    fn handle_between_doctype_public_and_system_identifiers_state(&mut self) {
        // Implementation for Between DOCTYPE public and system identifiers state
    }

    fn handle_after_doctype_system_keyword_state(&mut self) {
        // Implementation for After DOCTYPE system keyword state
    }

    fn handle_before_doctype_system_identifier_state(&mut self) {
        // Implementation for Before DOCTYPE system identifier state
    }

    fn handle_doctype_system_identifier_double_quoted_state(&mut self) {
        // Implementation for DOCTYPE system identifier (double-quoted) state
    }

    fn handle_doctype_system_identifier_single_quoted_state(&mut self) {
        // Implementation for DOCTYPE system identifier (single-quoted) state
    }

    fn handle_after_doctype_system_identifier_state(&mut self) {
        // Implementation for After DOCTYPE system identifier state
    }

    fn handle_bogus_doctype_state(&mut self) {
        // Implementation for Bogus DOCTYPE state
    }

    fn handle_cdata_section_state(&mut self) {
        // Implementation for CDATA section state
    }

    fn handle_cdata_section_bracket_state(&mut self) {
        // Implementation for CDATA section bracket state
    }

    fn handle_cdata_section_end_state(&mut self) {
        // Implementation for CDATA section end state
    }

    fn handle_character_reference_state(&mut self) {
        // Implementation for Character reference state
    }

    fn handle_named_character_reference_state(&mut self) {
        // Implementation for Named character reference state
    }

    fn handle_ambiguous_ampersand_state(&mut self) {
        // Implementation for Ambiguous ampersand state
    }

    fn handle_numeric_character_reference_state(&mut self) {
        // Implementation for Numeric character reference state
    }

    fn handle_hexadecimal_character_reference_start_state(&mut self) {
        // Implementation for Hexadecimal character reference start state
    }

    fn handle_decimal_character_reference_start_state(&mut self) {
        // Implementation for Decimal character reference start state
    }

    fn handle_hexadecimal_character_reference_state(&mut self) {
        // Implementation for Hexadecimal character reference state
    }

    fn handle_decimal_character_reference_state(&mut self) {
        // Implementation for Decimal character reference state
    }

    fn handle_numeric_character_reference_end_state(&mut self) {
        // Implementation for Numeric character reference end state
    }

    fn emit_token(&mut self, token: Token) {    
        match &token {
            Token::StartTag{..} => {
                self.last_start_tag_token = Some(token.clone());
            }
            _ => {
                
            }
        }
        println!("Emitting token: {:?}", token);
        self.tokens.push(token);
    }

    fn consume_next_input_char(&mut self) -> Option<u8>{
        let byte_character = self.input_stream.current_cpy();
        self.input_stream.advance();
        byte_character
    }

    fn reconsume_char(&mut self) {        
        self.input_stream.idx -= 1;
        self.input_stream.idx = max(self.input_stream.idx, 0);
    }

    fn emit_parse_error(&self, err: &str){
        eprint!("{err}\n");
    }

    fn add_attribute_to_current_tag_token(&mut self){
        let tag_name_exists = self.current_tag_attr_name_exist();
        if let Some(ref mut t) = self.current_tag_token {
            if tag_name_exists {
                self.emit_parse_error("attribute-name-existed");
            }else{
                t.add_attribute(self.current_tag_name.clone(), self.current_tag_value.clone());
                self.current_tag_name.clear();
                self.current_tag_value.clear();
            }

        } else {
            self.emit_parse_error("Token is None; cannot add attribute.");
        }
    }

    fn current_tag_attr_name_exist(&self) -> bool{
        if let Some(ref t) = self.current_tag_token {
            t.attribute_exists(&self.current_tag_name)
        } else {
            self.emit_parse_error("Token is None; cannot add attribute.");
            false
        }
    }
    fn emit_current_tag_token(&mut self) {

        if let Some(token) = self.current_tag_token.take() { 
            self.emit_token(token); 
        } else {
            eprintln!("No current tag token to emit.");
        }
    }
}

