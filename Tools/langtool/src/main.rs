use std::io;

mod section;

mod inifile;
use inifile::IniFile;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, long)]
    dry_run: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    CopyMissingLines {},
    CommentUnknownLines {},
    RemoveUnknownLines {},
    AddNewKey {
        section: String,
        key: String,
    },
    MoveKey {
        old: String,
        new: String,
        key: String,
    },
    RemoveKey {
        section: String,
        key: String,
    },
}

fn copy_missing_lines(reference_ini: &IniFile, target_ini: &mut IniFile) -> io::Result<()> {
    for reference_section in &reference_ini.sections {
        // Insert any missing full sections.
        if !target_ini.insert_section_if_missing(reference_section) {
            if let Some(target_section) = target_ini.get_section_mut(&reference_section.name) {
                for line in &reference_section.lines {
                    target_section.insert_line_if_missing(line);
                }

                //target_section.remove_lines_if_not_in(reference_section);
                target_section.comment_out_lines_if_not_in(reference_section);
            }
        }
    }
    Ok(())
}

fn deal_with_unknown_lines(
    reference_ini: &IniFile,
    target_ini: &mut IniFile,
    remove: bool,
) -> io::Result<()> {
    for reference_section in &reference_ini.sections {
        if let Some(target_section) = target_ini.get_section_mut(&reference_section.name) {
            if remove {
                target_section.remove_lines_if_not_in(reference_section);
            } else {
                target_section.comment_out_lines_if_not_in(reference_section);
            }
        }
    }
    Ok(())
}

fn move_key(target_ini: &mut IniFile, old: &str, new: &str, key: &str) -> io::Result<()> {
    if let Some(old_section) = target_ini.get_section_mut(old) {
        if let Some(line) = old_section.remove_line(key) {
            if let Some(new_section) = target_ini.get_section_mut(new) {
                new_section.insert_line_if_missing(&line);
            } else {
                println!("No new section {}", new);
            }
        } else {
            println!("No key {} in section {}", key, old);
        }
    } else {
        println!("No old section {}", old);
    }
    Ok(())
}

fn remove_key(target_ini: &mut IniFile, section: &str, key: &str) -> io::Result<()> {
    if let Some(old_section) = target_ini.get_section_mut(section) {
        let _ = old_section.remove_line(key);
    } else {
        println!("No section {}", section);
    }
    Ok(())
}

fn add_new_key(target_ini: &mut IniFile, section: &str, key: &str) -> io::Result<()> {
    if let Some(section) = target_ini.get_section_mut(section) {
        let _ = section.insert_line_if_missing(&format!("{} = {}", key, key));
    } else {
        println!("No section {}", section);
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    // TODO: Grab extra arguments from opt somehow.
    let args: Vec<String> = vec![]; //std::env::args().skip(1).collect();
    let mut filenames = args;

    let root = "../../assets/lang";
    let reference_ini_filename = "en_US.ini";

    let mut reference_ini = IniFile::parse(&format!("{}/{}", root, reference_ini_filename)).unwrap();

    if filenames.is_empty() {
        // Grab them all.
        for path in std::fs::read_dir(root).unwrap() {
            let path = path.unwrap();
            if path.file_name() == reference_ini_filename {
                continue;
            }
            let filename = path.file_name();
            let filename = filename.to_string_lossy();
            if !filename.ends_with(".ini") {
                continue;
            }
            filenames.push(path.file_name().to_string_lossy().to_string());
        }
    }

    for filename in filenames {
        let reference_ini = &reference_ini;
        if filename == "langtool" {
            // Get this from cargo run for some reason.
            continue;
        }
        let target_ini_filename = format!("{}/{}", root, filename);
        println!("Langtool processing {}", target_ini_filename);

        let mut target_ini = IniFile::parse(&target_ini_filename).unwrap();

        match opt.cmd {
            Command::CopyMissingLines {} => {
                copy_missing_lines(&reference_ini, &mut target_ini).unwrap();
            }
            Command::CommentUnknownLines {} => {
                deal_with_unknown_lines(&reference_ini, &mut target_ini, false).unwrap();
            }
            Command::RemoveUnknownLines {} => {
                deal_with_unknown_lines(&reference_ini, &mut target_ini, true).unwrap();
            }
            Command::AddNewKey {
                ref section,
                ref key,
            } => add_new_key(&mut target_ini, section, key).unwrap(),
            Command::MoveKey {
                ref old,
                ref new,
                ref key,
            } => {
                move_key(&mut target_ini, old, new, key).unwrap();
            }
            Command::RemoveKey {
                ref section,
                ref key,
            } => {
                remove_key(&mut target_ini, section, key).unwrap();
            }
        }

        if !opt.dry_run {
            target_ini.write().unwrap();
        }
    }

    println!("Langtool processing {}", reference_ini_filename);

    // Some commands also apply to the reference ini.
    match opt.cmd {
        Command::AddNewKey {
            ref section,
            ref key,
        } => {
            add_new_key(&mut reference_ini, section, key).unwrap();
        }
        Command::MoveKey {
            ref old,
            ref new,
            ref key,
        } => {
            move_key(&mut reference_ini, old, new, key).unwrap();
        }
        Command::RemoveKey {
            ref section,
            ref key,
        } => {
            remove_key(&mut reference_ini, section, key).unwrap();
        }
        _ => {}
    }

    if !opt.dry_run {
        reference_ini.write().unwrap();
    }
}
