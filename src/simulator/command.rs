use crate::simulator::CerealApp;

const HELP_MESSAGE: &str = include_str!("help.txt");

pub(crate) fn command(app: &mut CerealApp) {
    let cmd = &*app.command;
    let mut words = cmd.split_whitespace();
    match &*words.next().unwrap_or("").to_lowercase() {
        "h" | "help" => {
            app.command_output.push_str(HELP_MESSAGE);
            app.command_output.push('\n');
        }
        "as" => app.command_output.push_str("Unimplemented\n"),
        "b" | "break" => app.command_output.push_str("Unimplemented\n"),
        "bpred" => app.command_output.push_str("Unimplemented\n"),
        "c" | "continue" => app.command_output.push_str("Unimplemented\n"),
        "check" => app.command_output.push_str("Unimplemented\n"),
        "clear" => app.command_output.push_str("Unimplemented\n"),
        "counters" => app.command_output.push_str("Unimplemented\n"),
        "d" | "dump" => app.command_output.push_str("Unimplemented\n"),
        "goto" => app.command_output.push_str("Unimplemented\n"),
        "input" => app.command_output.push_str("Unimplemented\n"),
        "l" | "list" => app.command_output.push_str("Unimplemented\n"),
        "ld" | "load" => app.command_output.push_str("Unimplemented\n"),
        "loadhex" => app.command_output.push_str("Unimplemented\n"),
        "n" | "next" => app.command_output.push_str("Unimplemented\n"),
        "p" | "print" => app.command_output.push_str("Unimplemented\n"),
        "quit" => app.command_output.push_str("Unimplemented\n"),
        "reset" => app.command_output.push_str("Unimplemented\n"),
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