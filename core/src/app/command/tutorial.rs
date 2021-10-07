use super::{Command, CommandAlias, Runnable};
use crate::app::AppMeta;
use crate::world::npc::Gender;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TutorialCommand {
    Introduction,
    Inn,
    Save,
    Npc {
        inn_name: String,
    },
    NpcOther {
        inn_name: String,
    },
    SaveByName {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
        other_npc_name: String,
    },
    Journal {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    LoadByName {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Spell {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Weapons {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Roll {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Delete {
        npc_gender: Gender,
        npc_name: String,
    },
    AdjustTime {
        npc_gender: Gender,
        npc_name: String,
    },
    Time,
    Conclusion,
}

impl TutorialCommand {
    fn output(&self, command_output: Option<Result<String, String>>) -> Result<String, String> {
        let is_ok = if let Some(r) = &command_output {
            r.is_ok()
        } else {
            true
        };

        let mut output = command_output
            .unwrap_or_else(|| Ok(String::new()))
            .map_or_else(|e| e, |s| s);
        if !output.is_empty() {
            output.push_str("\n\n#");
        }

        match self {
            Self::Introduction => {}
            Self::Inn => output.push_str(include_str!("../../../../data/tutorial/00-intro.md")),
            Self::Save => output.push_str(include_str!("../../../../data/tutorial/01-inn.md")),
            Self::Npc { inn_name } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/02-save.md"),
                inn_name = inn_name,
            )),
            Self::NpcOther { .. } => {
                output.push_str(include_str!("../../../../data/tutorial/03-npc.md"))
            }
            Self::SaveByName {
                npc_gender,
                npc_name,
                other_npc_name,
                ..
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/04-npc-other.md"),
                npc_name = npc_name,
                other_npc_name = other_npc_name,
                their = npc_gender.their(),
            )),
            Self::Journal {
                inn_name,
                npc_gender,
                npc_name,
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/05-save-by-name.md"),
                inn_name = inn_name,
                npc_name = npc_name,
                them = npc_gender.them(),
            )),
            Self::LoadByName { .. } => {
                output.push_str(include_str!("../../../../data/tutorial/06-journal.md"))
            }
            Self::Spell { npc_name, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/07-load-by-name.md"),
                npc_name = npc_name,
            )),
            Self::Weapons {
                npc_gender,
                npc_name,
                ..
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/08-spell.md"),
                npc_name = npc_name,
                their = npc_gender.their(),
                them = npc_gender.them(),
                theyre_cap = npc_gender.theyre_cap(),
            )),
            Self::Roll {
                inn_name,
                npc_gender,
                npc_name,
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/09-weapons.md"),
                inn_name = inn_name,
                npc_name = npc_name,
                pull = if npc_gender == &Gender::Trans {
                    "pull"
                } else {
                    "pulls"
                },
                their = npc_gender.their(),
                they_cap = npc_gender.they_cap(),
                theyre = npc_gender.theyre(),
            )),
            Self::Delete {
                npc_gender,
                npc_name,
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/10-roll.md"),
                npc_name = npc_name,
                theyve = npc_gender.theyve(),
            )),
            Self::AdjustTime {
                npc_gender,
                npc_name,
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/11-delete.md"),
                npc_name = npc_name,
                them = npc_gender.them(),
                they_cap = npc_gender.they_cap(),
            )),
            Self::Time => {
                output.push_str(include_str!("../../../../data/tutorial/12-adjust-time.md"))
            }
            Self::Conclusion => {
                output.push_str(include_str!("../../../../data/tutorial/13-time.md"))
            }
        }

        if is_ok {
            Ok(output)
        } else {
            Err(output)
        }
    }
}

#[async_trait(?Send)]
impl Runnable for TutorialCommand {
    async fn run(&self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        let input_command = Command::parse_input_irrefutable(input, app_meta);

        let (result, next_command) = match self {
            Self::Introduction => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "next".to_string(),
                    "continue the tutorial".to_string(),
                    Self::Inn.into(),
                ));

                let next = Self::Inn;
                (next.output(None), Some(next))
            }
            Self::Inn if input == "next" => {
                let next = Self::Save;
                (next.output(None), Some(next))
            }
            Self::Save if input == "inn" => {
                let command_output = input_command.run(input, app_meta).await;

                if let Ok(output) = command_output {
                    let inn_name = output
                        .lines()
                        .next()
                        .unwrap()
                        .trim_start_matches(&[' ', '#'][..])
                        .to_string();

                    let next = Self::Npc { inn_name };
                    (next.output(Some(Ok(output))), Some(next))
                } else {
                    (command_output, Some(self.clone()))
                }
            }
            Self::Npc { inn_name }
                if input == "save"
                    || (input.starts_with("save ")
                        && input.ends_with(inn_name.as_str())
                        && input.len() == "save ".len() + inn_name.len()) =>
            {
                let next = Self::NpcOther {
                    inn_name: inn_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::NpcOther { inn_name } if input == "npc" => {
                let command_output = input_command.run(input, app_meta).await;

                if let Ok(output) = command_output {
                    let (npc_name, other_npc_name, npc_gender) = {
                        let mut lines_iter = output.lines();

                        let other_npc_name = lines_iter
                            .next()
                            .map(|s| s.trim_start_matches(&[' ', '#'][..]).to_string());
                        let npc_name = lines_iter
                            .find(|s| s.starts_with("~1~ "))
                            .and_then(|s| {
                                if let (Some(a), Some(b)) = (s.find('`'), s.rfind('`')) {
                                    s.get(a + 1..b)
                                } else {
                                    None
                                }
                            })
                            .map(|s| s.to_string());
                        let npc_gender = app_meta
                            .recent()
                            .iter()
                            .find(|t| t.name().value() == npc_name.as_ref())
                            .map(|t| t.gender());

                        (npc_name, other_npc_name, npc_gender)
                    };

                    if let (Some(npc_name), Some(other_npc_name), Some(npc_gender)) =
                        (npc_name, other_npc_name, npc_gender)
                    {
                        let next = Self::SaveByName {
                            inn_name: inn_name.clone(),
                            npc_gender,
                            npc_name,
                            other_npc_name,
                        };

                        (next.output(Some(Ok(output))), Some(next))
                    } else {
                        (Ok(output), Some(self.clone()))
                    }
                } else {
                    (command_output, Some(self.clone()))
                }
            }
            Self::SaveByName {
                inn_name,
                npc_gender,
                npc_name,
                ..
            } if input == "1"
                || input == npc_name
                || (input.starts_with("load ")
                    && input.ends_with(npc_name.as_str())
                    && input.len() == "load ".len() + npc_name.len()) =>
            {
                let command_output = input_command.run(input, app_meta).await;

                if let Ok(output) = command_output {
                    let next = Self::Journal {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (next.output(Some(Ok(output))), Some(next))
                } else {
                    (command_output, Some(self.clone()))
                }
            }
            Self::Journal {
                inn_name,
                npc_gender,
                npc_name,
            } if input == "save"
                || (input.starts_with("save ")
                    && input.ends_with(npc_name.as_str())
                    && input.len() == "save ".len() + npc_name.len()) =>
            {
                let next = Self::LoadByName {
                    inn_name: inn_name.clone(),
                    npc_gender: *npc_gender,
                    npc_name: npc_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::LoadByName {
                inn_name,
                npc_gender,
                npc_name,
            } if input == "journal" => {
                let next = Self::Spell {
                    inn_name: inn_name.clone(),
                    npc_gender: *npc_gender,
                    npc_name: npc_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::Spell {
                inn_name,
                npc_gender,
                npc_name,
            } if input == npc_name
                || (input.starts_with("load ")
                    && input.ends_with(npc_name.as_str())
                    && input.len() == "load ".len() + npc_name.len()) =>
            {
                let next = Self::Weapons {
                    inn_name: inn_name.clone(),
                    npc_gender: *npc_gender,
                    npc_name: npc_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::Weapons {
                inn_name,
                npc_gender,
                npc_name,
            } if input == "Fireball" => {
                let next = Self::Roll {
                    inn_name: inn_name.clone(),
                    npc_gender: *npc_gender,
                    npc_name: npc_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::Roll {
                npc_gender,
                npc_name,
                ..
            } if input == "weapons" => {
                let next = Self::Delete {
                    npc_gender: *npc_gender,
                    npc_name: npc_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::Delete {
                npc_gender,
                npc_name,
            } if input == "d20+4" => {
                let next = Self::AdjustTime {
                    npc_gender: *npc_gender,
                    npc_name: npc_name.clone(),
                };

                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::AdjustTime { npc_name, .. }
                if input.starts_with("delete ")
                    && input.ends_with(npc_name.as_str())
                    && input.len() == "delete ".len() + npc_name.len() =>
            {
                let next = Self::Time;
                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::Time if input == "+30m" => {
                let next = Self::Conclusion;
                (
                    next.output(Some(input_command.run(input, app_meta).await)),
                    Some(next),
                )
            }
            Self::Conclusion if ["time", "date", "now"].contains(&input) => (
                input_command.run(input, app_meta).await.map(|mut output| {
                    output.push_str("\n\n#");
                    output.push_str(include_str!("../../../../data/tutorial/99-conclusion.md"));
                    output
                }),
                None,
            ),
            _ => (
                Ok(include_str!("../../../../data/tutorial/xx-still-active.md").to_string()),
                Some(self.clone()),
            ),
        };

        if let Some(command) = next_command {
            app_meta
                .command_aliases
                .insert(CommandAlias::strict_wildcard(command.into()));
        }

        result
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            if input == "tutorial" {
                Some(TutorialCommand::Introduction)
            } else {
                None
            },
            Vec::new(),
        )
    }

    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        if "tutorial".starts_with(input) {
            vec![("tutorial".to_string(), "feature walkthrough".to_string())]
        } else {
            Vec::new()
        }
    }
}

impl fmt::Display for TutorialCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Introduction => write!(f, "tutorial"),
            _ => Ok(()),
        }
    }
}
