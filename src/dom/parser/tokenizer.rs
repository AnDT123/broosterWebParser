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
    },
    Comment {
        data: String,
    },
    Character {
        data: char,
    },
    EndOfFile,
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
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Tokenizer {
            input_stream: Stream::new(input),
            state: TokenizerState::Data,
            ret_state: TokenizerState::Data,
            current_tag_token: None,
            tokens: Vec::new(),
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
                eprintln!("Parse error: Unexpected null character");
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
                eprintln!("Parse error: Unexpected null character");
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
                eprintln!("Parse error: Unexpected null character");
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
                eprintln!("Parse error: Unexpected null character");
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
                eprintln!("Parse error: Unexpected null character");
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
                eprintln!("Parse error: Unexpected question mark instead of tag name");
                self.emit_token(Token::Comment{data:String::new()});
                self.state = TokenizerState::BogusComment;
                self.reconsume_char();
            }
            None => {
                eprintln!("Parse error: EOF before tag name");
                self.emit_token(Token::Character{data: '<'});
                self.emit_token(Token::EndOfFile);
            }
            Some(_) => {
                eprintln!("Parse error: Invalid first character of tag name");
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
                });
                self.state = TokenizerState::TagName;
                self.reconsume_char();
            }
            Some(b'>') => {
                eprintln!("Parse error: Missing end tag name");
                self.state = TokenizerState::Data;
            }
            None => {
                eprintln!("Parse error: EOF before tag name");
                self.emit_token(Token::Character{data: '<'});
                self.emit_token(Token::Character{data: '/'});
                self.emit_token(Token::EndOfFile);
            }
            Some(_) => {
                eprintln!("Parse error: Invalid first character of tag name");
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
                eprintln!("Parse error: Unexpected null character");
                if let Some(Token::StartTag { tag_name, .. }) = self.current_tag_token.as_mut() {
                    tag_name.push('\u{FFFD}');
                }
            }
            None => {
                eprintln!("Parse error: EOF in tag");
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
        // Implementation for RCDATA less-than sign state
    }

    fn handle_rcdata_end_tag_open_state(&mut self) {
        // Implementation for RCDATA end tag open state
    }

    fn handle_rcdata_end_tag_name_state(&mut self) {
        // Implementation for RCDATA end tag name state
    }

    fn handle_rawtext_less_than_sign_state(&mut self) {
        // Implementation for RAWTEXT less-than sign state
    }

    fn handle_rawtext_end_tag_open_state(&mut self) {
        // Implementation for RAWTEXT end tag open state
    }

    fn handle_rawtext_end_tag_name_state(&mut self) {
        // Implementation for RAWTEXT end tag name state
    }

    fn handle_script_data_less_than_sign_state(&mut self) {
        // Implementation for Script data less-than sign state
    }

    fn handle_script_data_end_tag_open_state(&mut self) {
        // Implementation for Script data end tag open state
    }

    fn handle_script_data_end_tag_name_state(&mut self) {
        // Implementation for Script data end tag name state
    }

    fn handle_script_data_escape_start_state(&mut self) {
        // Implementation for Script data escape start state
    }

    fn handle_script_data_escape_start_dash_state(&mut self) {
        // Implementation for Script data escape start dash state
    }

    fn handle_script_data_escaped_state(&mut self) {
        // Implementation for Script data escaped state
    }

    fn handle_script_data_escaped_dash_state(&mut self) {
        // Implementation for Script data escaped dash state
    }

    fn handle_script_data_escaped_dash_dash_state(&mut self) {
        // Implementation for Script data escaped dash dash state
    }

    fn handle_script_data_escaped_less_than_sign_state(&mut self) {
        // Implementation for Script data escaped less-than sign state
    }

    fn handle_script_data_escaped_end_tag_open_state(&mut self) {
        // Implementation for Script data escaped end tag open state
    }

    fn handle_script_data_escaped_end_tag_name_state(&mut self) {
        // Implementation for Script data escaped end tag name state
    }

    fn handle_script_data_double_escape_start_state(&mut self) {
        // Implementation for Script data double escape start state
    }

    fn handle_script_data_double_escaped_state(&mut self) {
        // Implementation for Script data double escaped state
    }

    fn handle_script_data_double_escaped_dash_state(&mut self) {
        // Implementation for Script data double escaped dash state
    }

    fn handle_script_data_double_escaped_dash_dash_state(&mut self) {
        // Implementation for Script data double escaped dash dash state
    }

    fn handle_script_data_double_escaped_less_than_sign_state(&mut self) {
        // Implementation for Script data double escaped less-than sign state
    }

    fn handle_script_data_double_escape_end_state(&mut self) {
        // Implementation for Script data double escape end state
    }

    fn handle_before_attribute_name_state(&mut self) {
        // Implementation for Before attribute name state
    }

    fn handle_attribute_name_state(&mut self) {
        // Implementation for Attribute name state
    }

    fn handle_after_attribute_name_state(&mut self) {
        // Implementation for After attribute name state
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
}

