use std::process::{Command, Output};

fn get_output_from_command(cmd: &str, args: Vec<&str>) -> Output {
    return Command::new(&cmd)
        .args(args)
        .output()
        .expect(format!("Failed to execute command: {}", &cmd).as_str());
}

fn get_stdout_as_string(output: Output) -> String {
    return match String::from_utf8(output.stdout) {
        Ok(x) => x.strip_suffix("\n").unwrap().to_string(),
        Err(e) => e.to_string()
    }
}

fn get_stderr_as_string(output: Output) -> String {
    return match String::from_utf8(output.stderr) {
        Ok(x) => x.strip_suffix("\n").unwrap().to_string(),
        Err(e) => e.to_string()
    }
}

pub fn get_result_as_string(cmd: &str, args:Vec<&str>) -> String {
    let output = get_output_from_command(cmd, args);
    match output.status.code() {
        Some(0) => get_stdout_as_string(output),
        _ => get_stderr_as_string(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::command::{get_result_as_string};

    #[test]
    fn test_get_output_from_command() {
        assert_eq!("Linux", get_result_as_string("uname", vec![]));
        assert_eq!("ls: cannot access '/dev/mouse': No such file or directory", 
            get_result_as_string("ls", vec!["/dev/mouse",]))
    }
}
