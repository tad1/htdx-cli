A cli tool for [htdx](https://tad1.dev/notes/Projects/htdx/htdx) that allows to search and display htdx notes.
<u>H</u>ow <u>t</u>o <u>d</u>o <u>x</u>? is a collection of minimal single-screen refferences on how to do tasks.

#### limitations
- currently support is limited to only my htdx repository
- only `zsh` is supported 

#### Upcoming features (todos)
- add support for any htdx repository
- add metadata support
- add support for other shells
- add markdown rendering


- preview note?

## Use
`htdx "my query string+<TAB>` - this will find all relevant notes, select a suggestion then press `Enter` to display a note
![Screencast_20260621_014922.webm](https://github.com/user-attachments/assets/6bb5fa6d-2f67-4cb6-8917-082814310093)

#### Add/Remove notes
notes are stores in `/usr/share/htdx/` any changes in this directory will affect the `htdx`

## Install
clone repo:
```sh
git clone https://github.com/tad1/htdx-cli.git
```

install as package
- Arch linux: `makepkg -si`
- debian: `makedeb -si`
- other - open issue `"Add support for htdx to [insert_name] distribution"` then I'll add it.

