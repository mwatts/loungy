/*
 *
 *  This source file is part of the Loungy open source project
 *
 *  Copyright (c) 2024 Loungy, Matthias Grandl and the Loungy project contributors
 *  Licensed under MIT License
 *
 *  See https://github.com/MatthiasGrandl/Loungy/blob/main/LICENSE.md for license information
 *
 */

use std::{collections::HashMap, time::Duration};

use gpui::*;

use crate::{
    commands::{RootCommand, RootCommandBuilder, RootCommands},
    components::{
        list::{nucleo::fuzzy_match, Accessory, Item, ItemBuilder, ListBuilder, ListItem},
        shared::{Icon, Img},
    },
    platform::{get_application_files, get_application_data},
    state::{Action, StateViewBuilder, StateViewContext},
    window::Window,
};

use super::numbat::{Numbat, NumbatWrapper};

#[derive(Clone)]
pub struct RootListBuilder;

impl StateViewBuilder for RootListBuilder {
    fn build(&self, context: &mut StateViewContext, cx: &mut WindowContext) -> AnyView {
        context
            .query
            .set_placeholder("Search for apps and commands...", cx);
        let numbat = Numbat::init(&context.query, cx);
        let commands = RootCommands::list(cx);
        ListBuilder::new()
            .interval(Duration::from_secs(60))
            .filter(move |this, cx| {
                let mut items = this.items_all.clone();
                items.append(&mut commands.clone());
                let query = this.query.view.upgrade();
                if query.is_none() {
                    return vec![];
                }
                let query = query.unwrap().read(cx).text.clone();
                let mut items = fuzzy_match(&query, items, false);
                if items.is_empty() {
                    if let Some(result) = numbat.read(cx).result.clone() {
                        items.push(
                            ItemBuilder::new(
                                "Numbat",
                                NumbatWrapper {
                                    inner: numbat.clone(),
                                },
                            )
                            .actions(vec![Action::new(
                                Img::default().icon(Icon::Copy),
                                "Copy",
                                None,
                                {
                                    move |this, cx: &mut WindowContext| {
                                        cx.write_to_clipboard(ClipboardItem::new(
                                            result.result.to_string(),
                                        ));
                                        this.toast.floating(
                                            "Copied to clipboard",
                                            Some(Icon::Clipboard),
                                            cx,
                                        );
                                        Window::close(cx);
                                    }
                                },
                                false,
                            )])
                            .build(),
                        );
                    }
                }
                items
            })
            .build(
                |_, _, _cx| {
                    {
                        let application_entries = get_application_files();

                        let mut apps = HashMap::<String, Item>::new();

                        for entry in application_entries {
                            // search for .icns in Contents/Resources
                            let data = get_application_data(&entry);
                            if data.is_none() {
                                continue;
                            }
                            let data = data.unwrap();
                            let app = ItemBuilder::new(
                                data.id.clone(),
                                ListItem::new(
                                    Some(data.icon.clone()),
                                    data.name.clone(),
                                    None,
                                    vec![Accessory::new(data.tag.clone(), None)],
                                ),
                            )
                            .keywords(vec![data.name.clone()])
                            .actions(vec![Action::new(
                                Img::default().icon(Icon::ArrowUpRightFromSquare),
                                format!("Open {}", data.tag.clone()),
                                None,
                                {
                                    let id = data.id.clone();

                                    #[cfg(target_os = "macos")]
                                    {
                                        let ex = data.tag == "System Setting";
                                        move |_, cx| {
                                            Window::close(cx);
                                            let id = id.clone();
                                            let mut command =
                                                std::process::Command::new("open");
                                            if ex {
                                                command.arg(format!(
                                                    "x-apple.systempreferences:{}",
                                                    id
                                                ));
                                            } else {
                                                command.arg("-b");
                                                command.arg(id);
                                            }
                                            let _ = command.spawn();
                                        }
                                    }
                                    #[cfg(target_os = "linux")]
                                    {
                                        move |_, cx| {
                                            Window::close(cx);
                                            let mut command =
                                            std::process::Command::new("gtk-launch");
                                            command.arg(id.clone());
                                            let _ = command.spawn();
                                        }
                                    }
                                },
                                false,
                            )])
                            .build();
                            apps.insert(data.id, app);
                        }
                        let mut apps: Vec<Item> = apps.values().cloned().collect();
                        apps.sort_unstable_by_key(|a| a.get_keywords()[0].clone());
                        Ok(Some(apps))
                    }
                },
                context,
                cx,
            )
            .into()
    }
}

pub struct LoungyCommandBuilder;

impl RootCommandBuilder for LoungyCommandBuilder {
    fn build(&self, _cx: &mut WindowContext) -> RootCommand {
        RootCommand::new(
            "loungy",
            "Loungy",
            "Preferences",
            Icon::Rocket,
            vec!["Settings"],
            None,
            Box::new(|actions, cx| {
                actions.toast.error("Preferences not yet implemented", cx);
            }),
        )
    }
}
