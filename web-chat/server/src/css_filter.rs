use rustc_hash::FxHashSet;
use cssparser::{BasicParseErrorKind, DeclarationParser, ParseError, Parser, ParserInput, ToCss, Token};

pub fn default_parser_conf() -> ParserConfig {
    ParserConfig {
        allow_nest_rules: true,
        allowed_fns: FxHashSet::from_iter([
            "rgb", "rgba",
            "hsl", "hsla", "hwb",
            "lab", "lch",
            "oklab", "oklch",
            "color-mix",

            "light-dark",

            "abs", "log", // Math
            "min", "max", "mod",
            "pow", "rem", "round",
            "sign", "sqrt",
            "calc",

            "attr",
            "var",

            // Filters
            "hue-rotate", "invert", "opacity", "brightness", "blur",
            "contrast", "drop-shadow", "greyscale", "saturate", "sepia",

            "linear-gradient",
            "radial-gradient",
            "conic-gradient",
            "repeating-linear-gradient",
            "repeating-radial-gradient",
            "repeating-conic-gradient",
        ].into_iter().map(String::from)),
        allowed_props: FxHashSet::from_iter([
            "color",
            "color-scheme",
            "background",

            // "border",
            "outline",
            "box-shadow",
            "content",

            "vertical-align",

            "cursor",
            "pointer-events",
            "user-select",

            "filter",
            "mix-blend-mode", "isolate",

            // "font-size",
            "font-family", "font-style", "font-stretch",
            "font-variant", "font-weight",

            "text-align",
            "text-align-last",
            "text-box",
            "text-box-edge",
            "text-box-trim",
            "text-combine-upright",
            "text-decoration",
            "text-decoration-color",
            "text-decoration-line",
            "text-decoration-skip",
            "text-decoration-skip-ink",
            "text-decoration-style",
            "text-decoration-thickness",
            "text-emphasis",
            "text-emphasis-color",
            "text-emphasis-position",
            "text-emphasis-style",
            "text-indent",
            "text-justify",
            "text-orientation",
            "text-overflow",
            "text-rendering",
            "text-shadow",
            "text-size-adjust",
            "text-spacing-trim",
            "text-transform",
            "text-underline-offset",
            "text-underline-position",
            "text-wrap",
            "text-wrap-mode",
            "text-wrap-style",

            "white-space",
            "white-space-collapse",
            "word-break",
            "word-spacing",
            "word-wrap",
            "overflow-wrap",
        ].into_iter().map(String::from)),
    }
}

pub struct ParserConfig {
    pub allow_nest_rules: bool,
    pub allowed_fns: FxHashSet<String>,
    pub allowed_props: FxHashSet<String>,
}

pub fn parse_body<'a>(s: &'a str, config: &'_ ParserConfig) -> Result<String, ParseError<'a, ()>> {
    let mut input = ParserInput::new(&s);
    let mut parser = Parser::new(&mut input);
    let mut rule_parser = RuleParser(config);

    let mut body = String::with_capacity(s.len() + 16);
    let rules = cssparser::RuleBodyParser::new(&mut parser, &mut rule_parser)
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
    serialize_body(&mut body, rules.into_iter(), config);
    Ok(body)
}

fn serialize_body<'a>(out: &mut String, lines: impl Iterator<Item = Line<'a>>, config: &'_ ParserConfig) {
    for line in lines {
        match line {
            Line::QualifiedRule(selector, lines) => {
                if !selector.nested { continue }
                if !config.allow_nest_rules { continue }
                out.push_str(&selector.value);
                out.push_str(" {\n");
                serialize_body(out, lines.into_iter(), config);
                out.push_str("}\n");
            }
            Line::Declaration(name, value) => {
                if !name.is_empty() {
                    out.push_str(&name);
                    out.push(':');
                    out.push_str(&value);
                    out.push_str(";\n");
                }
            }
        }
    }
}

#[derive(Debug)]
struct Selector { value: String, nested: bool }

#[derive(Debug)]
enum Line<'i> {
    QualifiedRule(Selector, Vec<Line<'i>>),
    Declaration(cssparser::CowRcStr<'i>, String),
}

struct RuleParser<'a>(&'a ParserConfig);

impl<'a> cssparser::RuleBodyItemParser<'a, Line<'a>, ()> for RuleParser<'_> {
    fn parse_declarations(&self) -> bool { true }
    fn parse_qualified(&self) -> bool { true }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser<'_> {
    type Prelude = ();
    type AtRule = Line<'i>;
    type Error = ();
}

fn parse_selector_token<'i, 't>(out: &mut String, input: &mut Parser<'i, 't>, token: &Token<'i>) -> Result<(), ParseError<'i, ()>> {
    match token {
        Token::Function(_f) => {
            let Ok(_) = token.to_css(out) else {
                let err = cssparser::BasicParseErrorKind::UnexpectedToken(token.clone());
                return Err(input.new_error::<()>(err));
            };
            input.parse_nested_block(|input_inner| {
                // let mut first = true;
                loop {
                    match input_inner.next_including_whitespace() {
                        Ok(token) => {
                            // if !first && token != &Token::Comma {
                            //     out.push(' ');
                            // }
                            let token = token.clone();
                            parse_selector_token(out, input_inner, &token)?;
                            // first = false;
                        }
                        Err(e) if e.kind == BasicParseErrorKind::EndOfInput => break Ok(()),
                        Err(e) => return Err(e.into()),
                    }
                }
            })?;
            out.push(')');
            Ok(())
        },
        _ => {
            let Ok(_) = token.to_css(out) else {
                let err = cssparser::BasicParseErrorKind::UnexpectedToken(token.clone());
                return Err(input.new_error(err));
            };
            Ok(())
        }
    }
}

impl<'i> cssparser::QualifiedRuleParser<'i> for RuleParser<'_> {
    type Prelude = Selector;
    type QualifiedRule = Line<'i>;
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        input.skip_whitespace();
        let pos = input.state();
        let nested = input.expect_delim('&').is_ok();
        input.reset(&pos);
        let mut string = String::new();
        while let Ok(token) = input.next_including_whitespace().cloned() {
            parse_selector_token(&mut string, input, &token)?;
        }
        Ok(Selector { value: string, nested })
    }
    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let _ = start;
        let values = cssparser::RuleBodyParser::new(input, self)
            .map(|r| r.map_err(|(e, _)| e))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Line::QualifiedRule(prelude, values))
    }
}

fn parse_exprs<'i, 't>(config: &ParserConfig, input: &mut Parser<'i, 't>, t: &Token<'i>, value: &mut String) -> Result<(), cssparser::ParseError<'i, ()>> {
    match t {
        Token::BadString(_) | Token::BadUrl(_) => {
            let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
            return Err(input.new_error(err));
        }
        Token::Function(name) => {
            if !config.allowed_fns.contains(&**name) {
                return Err(input.new_error(cssparser::BasicParseErrorKind::UnexpectedToken(t.clone())));
            }
            let Ok(_) = t.to_css(&mut *value) else {
                let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                return Err(input.new_error::<()>(err));
            };
            input.parse_nested_block(|p| {
                let mut first = true;
                loop {
                    match p.next() {
                        Ok(t) => {
                            if !first && t != &Token::Comma {
                                value.push(' ');
                            }
                            let t = t.clone();
                            parse_exprs(config, p, &t, &mut *value)?;
                            first = false;
                        }
                        Err(e) if e.kind == BasicParseErrorKind::EndOfInput => break Ok(()),
                        Err(e) => return Err(e.into()),
                    }
                }
            })?;
            value.push(')');
        }
        Token::UnquotedUrl(_) => {
            if !config.allowed_fns.contains("url") {
                return Err(input.new_error(cssparser::BasicParseErrorKind::UnexpectedToken(t.clone())));
            }
            let Ok(_) = t.to_css(&mut *value) else {
                let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                return Err(input.new_error(err));
            };
        }
        _ => {
            let Ok(_) = t.to_css(&mut *value) else {
                let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                return Err(input.new_error(err));
            };
        }
    }
    Ok(())
}

impl <'i> DeclarationParser<'i> for RuleParser<'_> {
    type Declaration = Line<'i>;
    type Error = ();

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        let mut value = String::new();
        loop {
            let t = match input.next() {
                Err(e) if e.kind == cssparser::BasicParseErrorKind::EndOfInput => {
                    &Token::Semicolon
                }
                t => t?,
            };
            match t {
                Token::Semicolon => { 
                    if value.chars().all(char::is_whitespace) {
                        return Ok(Line::Declaration("".into(), String::new()));
                    }
                    break;
                }
                _ => {
                    if !value.is_empty() && value.chars().last() != Some(' ') {
                        value.push(' ');
                    }
                    let t = t.clone();
                    parse_exprs(self.0, input, &t, &mut value)?;
                }
            }
        }
        if value.chars().all(char::is_whitespace) {
            Err(input.new_error(cssparser::BasicParseErrorKind::EndOfInput))
        } else if !self.0.allowed_props.contains(&*name) {
            Err(input.new_error(cssparser::BasicParseErrorKind::QualifiedRuleInvalid))
        } else {
            Ok(Line::Declaration(name, value))
        }
    }
}
