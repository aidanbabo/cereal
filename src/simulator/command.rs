use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use crate::simulator::{loader, CerealApp, Machine};

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
        "as" => app.command_output.push_str("Unimplemented\n"),
        "b" | "break" => app.command_output.push_str("Unimplemented\n"),
        "bpred" => app.command_output.push_str("Unimplemented\n"),
        "c" | "continue" => app.command_output.push_str("Unimplemented\n"),
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
                app.command_output.push_str(&*output);
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
            app.command_output.push_str("System reset\n");
        },
        "s" | "step" => app.command_output.push_str("Unimplemented\n"),
        "script" => app.command_output.push_str("Unimplemented\n"),
        "set" => app.command_output.push_str("Unimplemented\n"),
        "stop" => app.command_output.push_str("Unimplemented\n"),
        "trace" => app.command_output.push_str("Unimplemented\n"),
        unknown => {
            app.command_output.push_str("Unknown command: ");
            app.command_output.push_str(unknown);
            app.command_output.push('\n');
        }
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