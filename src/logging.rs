use colored::Colorize;

pub fn log_printer(severity: &str, message: &str, line: u32, file: &str) {
    let log_severity = match severity {
        "error" => "ERROR".to_string().red().bold(),
        "warning" => "WARN".to_string().yellow().bold(),
        "info" => "INFO".to_string().green().bold(),
        _ => "INFO".to_string().green().bold(),
    };

    println!("{} {}, {}: {}", log_severity, message, line, file);
}
