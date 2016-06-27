#![feature(plugin_registrar, rustc_private)]

extern crate chrono;
extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use chrono::{DateTime, UTC};
use rustc_plugin::Registry;
use std::collections::HashMap;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::parse::token;
use syntax::ptr::P;

fn decorator(cx: &mut ExtCtxt, sp: Span, meta_item: &ast::MetaItem,
             item: &Annotatable, _push: &mut FnMut(Annotatable))
{
    let args = match meta_item.node {
        ast::MetaItemKind::List(ref name, ref args) if name == "fixbomb" => args,
        _ => {
            cx.span_warn(sp, "Bad attribute usage.");
            return;
        }
    };

    let mut date: Option<DateTime<UTC>> = None;
    let mut message: Option<token::InternedString> = None;

    // Parse arguments
    for arg in args {
        match arg.node {
            ast::MetaItemKind::NameValue(ref name, ref value) if name == "date" => {
                match value.node {
                    ast::LitKind::Str(ref s, _) => {
                        match s.parse() {
                            Ok(value) => { date = Some(value); }
                            Err(e) => cx.span_warn(value.span, &format!("Invalid date: {}", e)),
                        }
                    }
                    _ => {}
                }
            }
            ast::MetaItemKind::NameValue(ref name, ref value) if name == "message" => {
                match value.node {
                    ast::LitKind::Str(ref s, _) => {
                        message = Some(s.clone());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Trigger the bomb, if expired.
    if let Some(date) = date {
        let now = UTC::now();

        if now >= date {
            let message = match message {
                Some(message) => format!("Fixbomb triggered: {}", message),
                None => "Fixbomb triggered!".into(),
            };
            cx.span_err(sp, &message);
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("fixbomb"),
                                  SyntaxExtension::MultiDecorator(Box::new(decorator)));
}
