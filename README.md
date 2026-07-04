# Jewfetch

jewfetch is a fetching tool written in rust
the name is for entertainment purposes only

![jewfetch](pics/default.png)

# Installation / Uninstallation

```bash
chmod +x start.sh
./start.sh
```
choose 1 if you want to install by building (recommended)
choose 2 if you want to uninstall

# Configuration

the path that contains config files is `~/.config/jewfetch`

- `config.json`
you can set color, ascii art and the components to be displayed and their order from here

commands are stored in `~/.config/jewfetch/commands`
you can create new command to create new component here
after you create new command add new object to the components section in `~/.config/jewfetch/config.json`

ascii arts are stored in `~/.config/jewfetch/ascii-arts`
if you want to create a custom ascii art, create new file here for example: `art.txt`
then set the ascii section into the name of your file that you created previously for example: `"ascii":"art"`

you can also select the color whatever you want from the section color
available colors: `black,red,green,yellow,blue,purple,cyan,white`
default color is blue.

# Disclaimer

this project was created solely for entertainment purposes, and there is no racism involved.

