# bgl
music manager written in rust

### Installation
use the following `install.sh` script to install the binary to your machine. This install script copies the bgl binary into `/usr/local/bin` and creates the config directory `~/.config/bagel`

#### Step 1
Run the following install script like so

```bash
./install.sh
installation complete
```

#### Step 2
Install the following dependencies

##### pacman
```bash
sudo pacman -S rofi socat mpv youtube-dl jq
```
##### apt
```bash
sudo apt install rofi socat mpv youtube-dl jq
```

#### Step 3
Run the following command to get the `playlist.json` file

```bash
bgl pull
```

#### Step 4
Add the slack auth token to sendMsg auth variable
```bash
auth="xoxb-xxxxxxxxx-xxxxxx-xxxxxxxxxx"
if [ "$1" == "-updateBgl" ]; then
	curl -X POST \
	     -H "Authorization: Bearer "$auth"" \
	     -H 'Content-type: application/json; charset=utf-8' \
	    --data '{ "channel" : "C01EA8FBCEN", "text" : "A Change Was Made To The Bagel Playlist. Do a `bagel pull` to recieve these changes", "username": "sickbeats" }' \
	https://slack.com/api/chat.postMessage
fi
```

#### Step 5
use the built in help menu guide you through using the application

```bash
bgl -h
--------------------------
syntax : bagel [arg]

possible args include :
  [push] : get latest playlist copy
  [pull] : commit playlist changes to live
  [collection] : allows selection of a music track
```
