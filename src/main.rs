use notify_rust::{Notification, Hint};
use std::env;
use std::process::Command;

fn send_notification(volume: u32, muted: bool) {
    let (summary, icon): (String, &str) = if muted {
        ("Muted".to_string(), "audio-volume-muted")
    } else if volume == 0 {
        ("Volume: 0%".to_string(), "audio-volume-low")
    } else if volume < 30 {
        (format!("Volume: {}%", volume), "audio-volume-low")
    } else if volume < 70 {
        (format!("Volume: {}%", volume), "audio-volume-medium")
    } else {
        (format!("Volume: {}%", volume), "audio-volume-high")
    };

    let mut notification = Notification::new();
    notification
        .summary(&summary)
        .id(5555)
        .timeout(1000)
        .icon(icon)
        .hint(Hint::Category("volume".to_string()))
        .hint(Hint::CustomInt("value".to_string(), volume as i32));

    let _ = notification.show();
}

fn get_volume(verbose: bool) -> (u32, bool) {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output();

    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if verbose {
                eprintln!("[volctl] wpctl get-volume output: {}", output_str);
            }
            let mut volume = 0;
            let mut muted = false;

            for token in output_str.trim().split_whitespace() {
                if token == "[MUTED]" {
                    muted = true;
                } else if let Ok(v) = token.parse::<f32>() {
                    volume = (v * 100.0).round() as u32;
                }
            }

            (volume, muted)
        }
        Err(e) => {
            eprintln!("[volctl] Error getting volume: {}", e);
            if verbose {
                eprintln!("[volctl] Note: could not capture stderr for get-volume");
            }
            (0, false)
        }
    }
}

fn set_volume(change: i32, verbose: bool) {
    let op = if change > 0 { "+" } else { "-" };
    let amount = format!("{}%{}", change.abs(), op);

    if verbose {
        eprintln!("[volctl] executing wpctl with amount {}", amount)
    }

    match Command::new("wpctl")
        .args(["set-volume", "-l", "1.0", "@DEFAULT_AUDIO_SINK@", &amount])
        .output() {
            Ok(output) if output.status.success() => {
                let (volume, muted) = get_volume(verbose);
                send_notification(volume, muted);
            }
            Ok(output) => {
                eprintln!("[volctl] wpctl exited with status: {}", output.status);
                if verbose {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("[volctl] wpctl stderr: {}", stderr);
                }
            }
            Err(e) => {
                eprintln!("[volctl] Failed to set volume: {}", e);
            }
        }
}

fn toggle_mute(verbose: bool) {
    match Command::new("wpctl")
        .args(["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"])
        .output() {
            Ok(output) if output.status.success() => {
                let (volume, muted) = get_volume(verbose);
                send_notification(volume, muted);
            }
            Ok(output) => {
                eprintln!("[volctl] wpctl exited with status: {}", output.status);
                if verbose {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("[volctl] wpctl stderr: {}", stderr);
                }
            }
            Err(e) => {
                eprintln!("[volctl] Failed to toggle mute: {}", e);
            }
        }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: volctl [up|down|mute] [amount] [--verbose]");
        return;
    }

    if args[1] == "--help" || args[1] == "-h" {
        println!("Usage: volctl [up|down|mute] [amount] [--verbose]");
        return;
    }

    let verbose = args.contains(&"--verbose".to_string());

    match args[1].as_str() {
        "up" => {
            let step = args.iter()
                .skip(2)
                .find(|v| !v.starts_with('-'))
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(5);
            set_volume(step, verbose);
        }
        "down" => {
            let step = args.iter()
                .skip(2)
                .find(|v| !v.starts_with('-'))
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(5);
            set_volume(-step, verbose);
        }
        "mute" => {
            toggle_mute(verbose);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

