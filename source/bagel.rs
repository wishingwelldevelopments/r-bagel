use std::env;
use std::process::Command;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let home = env::var("HOME").unwrap_or("none".to_string());
    let mut file = File::open("".to_owned()+&home+"/.config/bagel/api_key").expect("Unable to open the file");
    let mut api_key = String::new();
    file.read_to_string(&mut api_key).expect("Unable to read the file");
    api_key.pop();

    for argument in env::args() {
        if argument == "-h" {
            println!("bgl v1.7.1");
            println!("--------------------------");
            println!("syntax : bagel [arg]");
            println!("");
            println!("possible args include :");
            println!("  [push] : get latest playlist copy");
            println!("  [pull] : commit playlist changes to live");
            println!("  [collection] : allows selection of a music track");
        }
        if argument == "collection" {
            let track = Command::new("sh")
                .arg("-c")
                .arg("cat ~/.config/bagel/playlist.json | jq -r '.tracks | .[] | .name + \" - \"+ .genre + \" (\" + .url +\")\"' | rofi -font 'Source Code Pro 9' -dmenu -p 'Select a Track' -i | cut -d '(' -f2 | cut -d ')' -f1")
                .output()
                .expect("failed to connect to db");

            let track = String::from_utf8_lossy(&track.stdout);

            if track != "" {
                let playing = Command::new("sh")
                    .arg("-c")
                    .arg("echo ".to_owned()+&track+" | cut -d '(' -f2 | cut -d ')' -f1")
                    .output()
                    .expect("failed to connect to db");

                let playing = String::from_utf8_lossy(&playing.stdout);
                println!("Trying : {}", playing);

                Command::new("sh")
                    .arg("-c")
                    .arg("killall mpv; mpv --no-video --shuffle --idle --input-ipc-server=/tmp/mpvsocket ".to_owned()+&playing)
                    .spawn()
                    .expect("failed to connect to db");

                println!("My Work Here Is Done. Enjoy!");

            }else{
                println!("no selection made");
            }
        }
        if argument == "push" {
            println!("Pushing Changes to Live ...");
            Command::new("curl")
                .arg("--silent")
                .arg("--output")
                .arg("/dev/null")
                .arg("-X")
                .arg("PUT")
                .arg("-d")
                .arg("@".to_owned()+&home+"/.config/bagel/playlist.json")
                .arg("https://bagel-1d0b2.firebaseio.com/.json?auth=".to_owned()+&api_key)
                .status()
                .expect("failed to connect to db");

            Command::new("sh")
                .arg("-c")
                .arg("sendMsg -updateBgl")
                .spawn()
                .expect("failed to connect to db");
        }
        if argument == "pull" {
            println!("Downloading Source...");
            let status = Command::new("bash")
                .arg("-c")
                .arg("vimdiff <(curl --silent https://bagel-1d0b2.firebaseio.com/.json?auth=".to_owned()+&api_key+" | jq) ~/.config/bagel/playlist.json")
                .status()
                .expect("failed to connect to db");

            if status.to_string() == "exit code: 0" {
                println!("Pull Successful");
            }
        }
    }
}
