use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use crate::simulator::{loader, CerealApp, ExecutionState, Machine};

static HELP_MESSAGES: Lazy<BTreeMap<&str, &str>> = Lazy::new(|| {
    let mut map = BTreeMap::new();
    map.insert("as", "as usage: as <outfilename> <infilename>+");
    map.insert("b", "b[reak] usage: b[reak] [ set | clear ] [ mem_addr | label ]");
    map.insert("bpred", "bpred usage: bpred <size>");
    map.insert("c", "c[ontinue] usage: c[ontinue]");
    map.insert("check", "check usage: check [ count | cumulative | reset | PC | reg | PSR | MPR | mem_addr | label | N | Z | P ] [ mem_addr | label ] [ value | label ]");
    map.insert("clear", "clear usage: clear");
    map.insert("counters", "counters usage: counters");
    map.insert("d", "d[ump] usage: d[ump] [-check | -coe | -readmemh | -disasm] from_mem_addr to_mem_addr dumpfile");
    map.insert("goto", "goto usage: goto [<addr>|<label>]");
    map.insert("h", "h[elp] usage: h[elp] [command]");
    map.insert("input", "input usage: input <filename>");
    map.insert("l", "l[ist] usage: l[ist] [ addr1 | label1 [addr2 | label2] ]");
    map.insert("ld", "l[oa]d usage: l[oa]d <filename>");
    map.insert("loadhex", "loadhex usage: loadhex hexfile");
    map.insert("next", "n[ext] usage: n[ext]");
    map.insert("p", "p[rint] usage: p[rint]");
    map.insert("quit", "quit usage: quit");
    map.insert("reset", "reset usage: reset");
    map.insert("s", "s[tep] usage: s[tep]"); // abbreviations for correct sorting
    map.insert("script", "script usage: script <filename>");
    map.insert("set", "set usage: set [ PC | reg | PSR | MPR | mem_addr | label ] [ mem_addr | label ] [ value | N | Z | P ]");
    map.insert("stop", "stop usage: stop");
    map.insert("trace", "trace usage: trace [on <trace-file> | off]");
    map
});

pub(crate) fn command(app: &mut CerealApp) {
    let cmd = &*app.command;
    let mut words = cmd.split_whitespace();
    match &*words.next().unwrap_or("").to_lowercase() {
        "h" | "help" => {
            for (_, &help) in HELP_MESSAGES.iter() {
                app.command_output.push_str(help);
                app.command_output.push('\n');
            }
        }
        "as" => {
            let output = assemble(words);
            app.command_output.push_str(&output);
        }
        "b" | "break" => {
            let Some(kind) = words.next() else {
                app.command_output.push_str(HELP_MESSAGES["b"]);
                app.command_output.push('\n');
                return;
            };

            enum Kind { Set, Clear }

            let kind = match &*kind.to_lowercase() {
                "set" => Kind::Set,
                "clear" => Kind::Clear,
                _ => {
                    app.command_output.push_str(HELP_MESSAGES["b"]);
                    app.command_output.push('\n');
                    return;
                }
            };

            let Some(label) = words.next() else { 
                app.command_output.push_str(HELP_MESSAGES["b"]);
                return;
            };

            let Some(&addr) = app.machine.symbols.get(label) else {
                app.command_output.push_str(&format!("Error: Invalid label ('{}')\n", label));
                return;
            };

            let verb = match kind {
                Kind::Set => {
                    use std::collections::btree_map::Entry;
                    if let Entry::Vacant(e) = app.breakpoints.entry(addr) {
                        e.insert(label.to_string());
                        "set"
                    } else {
                        app.breakpoints.remove(&addr);
                        "removed"
                    }
                }
                Kind::Clear => {
                    app.breakpoints.remove(&addr);
                    "cleared"
                }
            };
            app.command_output.push_str(&format!("Breakpoint {verb} at x{addr:04X}\n"));
        },
        "bpred" => app.command_output.push_str("Unimplemented\n"),
        "c" | "continue" => {
            app.execution_state = ExecutionState::Running;
            app.command_output.push_str("use the 'stop' command to interrupt execution\n");
        },
        "check" => app.command_output.push_str("Unimplemented\n"),
        "clear" => app.command_output.clear(),
        "counters" => app.command_output.push_str("Unimplemented\n"),
        "d" | "dump" => app.command_output.push_str("Unimplemented\n"),
        "goto" => app.command_output.push_str("Unimplemented\n"),
        "input" => app.command_output.push_str("Unimplemented\n"),
        "l" | "list" => app.command_output.push_str("Unimplemented\n"),
        "ld" | "load" => {
            if let Some(filename) = words.next() {
                let output = load_from_filename(filename, &mut app.machine);
                app.command_output.push_str(&output);
                app.command_output.push('\n');
            } else {
                app.command_output.push_str(HELP_MESSAGES["ld"]);
                app.command_output.push('\n');
            }
        }
        "loadhex" => app.command_output.push_str("Unimplemented\n"),
        "n" | "next" => app.command_output.push_str("Unimplemented\n"),
        "p" | "print" => app.command_output.push_str("Unimplemented\n"),
        "quit" => app.command_output.push_str("Unimplemented\n"),
        "reset" => {
            app.machine.reset();
            app.breakpoints.clear();
            app.command_output.push_str("System reset\n");
        },
        "s" | "step" => app.command_output.push_str("Unimplemented\n"),
        "script" => app.command_output.push_str("Unimplemented\n"),
        "set" => app.command_output.push_str("Unimplemented\n"),
        "stop" => {
            app.execution_state = ExecutionState::Suspended;
            app.command_output.push_str(&format!("Stopped at x{:04X}\n", app.machine.pc));
        },
        "trace" => {
            match words.next().map(str::to_lowercase).as_deref() {
                Some("on") => {
                    let Some(filename) = words.next() else {
                        app.command_output.push_str(HELP_MESSAGES["trace"]);
                        app.command_output.push('\n');
                        return;
                    };

                    let Ok(f) = std::fs::File::create(filename) else {
                        app.command_output.push_str("Unable to open file\n");
                        return;
                    };
                    app.trace = Some(Box::new(std::io::BufWriter::new(f)));
                    app.command_output.push_str("Trace is on.\n");
                },
                Some("off") => {
                    app.trace = None;
                    app.command_output.push_str("Trace is off.\n");
                },
                _ => {
                    app.command_output.push_str(HELP_MESSAGES["trace"]);
                    app.command_output.push('\n');
                },
            }
        },
        unknown => {
            app.command_output.push_str("Unknown command: ");
            app.command_output.push_str(unknown);
            app.command_output.push('\n');
        }
    }
}

fn assemble<'a>(mut words: impl Iterator<Item = &'a str>) -> String {
    let Some(mut output_path) = words.next().map(String::from) else { return HELP_MESSAGES["as"].to_string(); };
    output_path.push_str(".obj");
    let input_paths = words.map(|f| format!("{f}.asm").into()).collect::<Vec<_>>();
    if input_paths.is_empty() {
        return HELP_MESSAGES["as"].to_string();
    }

    let options = crate::Options {
        input_paths,
        output_path: output_path.into(),
        debug_info: true,
    };
    match crate::compile(options) {
        Ok(()) => "Assembly completed without errors or warnings\n".to_string(),
        Err(()) => "Assemble error\n".to_string(),
    }
}

fn load_from_filename(filename: &str, machine: &mut Machine) -> String {
    let filename = &format!("{filename}.obj");
    let bytes = match std::fs::read(filename) {
        Ok(bytes) => bytes,
        Err(_) => {
            return format!("Cannot find file '{filename}'");
        }
    };
    match loader::load(&bytes, machine, None) {
        Ok(()) => format!("Loading object file {filename}: code and data ...  symbols ...  file and line numbers ... "),
        Err(e) => format!("Error loading file '{filename}': {e:?}"),
    }
}