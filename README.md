# r-bagel
music manager written in rust

### Installation
use the following `install.sh` script to install the binary to your machine. This install script copies the r-bagel binary into `/usr/local/bin` and creates the config directory `~/.config/bagel`

#### Step 1
Run the following install script like so

```bash
./install.sh
installation complete
```

#### Step 2
Run the following command to get the `playlist.json` file

```bash
r-bagel pull
```

#### Step 3
Install the following dependencies

```bash
sudo pacman -S rofi socat mpv youtube-dl jq
```

#### Step 4
Install the following dependencies

```bash
sudo pacman -S rofi socat mpv youtube-dl jq
```

#### Step 5
use the built in help menu guide you through using the application

```bash
r-bagel help
--------------------------
syntax : bagel [arg]
possible args include :
    [push] : get latest playlist copy
    [pull] : commit playlist changes to live
    [help] : prints this message
    [collection] : allows selection of a music track
```
