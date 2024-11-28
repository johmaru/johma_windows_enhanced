# johma_windows_enhanced


## General Usage
jwe <Options> <--args>

## Now Support Command

These commands are incomplete

<span style="color: red;">All command options have a --help command</span> 

### CPU Command

- ```jwe cpu <option>```
  - --all(short -a)
  - --usage(short -u)
  - --frequency(short -f)
  - --all-pid

### Memory Command

- ```jwe mem show <option>```
    - --all(short -a)
    - --free(short -f)
    - --used(short -u)
    - --available(short -v)

### LS Command

- ```jwe ls```

### Browser Command

- ```jwe browser show <option>```
  - --all(short -a)
  - --set
  - --reset
  - --set-search
- ```jwe browser fav <option>```
  - --add-favorite(short -a)
  - --remove-favorite(short -r)
  - --list-favorite(short -l)
  - --open-favorite (content) (short -o)  
- ```jwe browser search <message>``` 

### Open Command
- ```jwe open <command>```
  - ```taskm```
  - ```env```
  - ```Appdata```
  - ```Local```
  - ```local-low```
  - ```roaming```
  - ```There```
  - ```all-sid```

### Remove Command
- ```jwe rm <file-name>```

### Version Command
 - Does not have any arguments

### Explorer Command
- ```jwe expl <command>```
  - ```reflesh```

### Process Command
- ```jwe proc show <option>```
  - ```all```
- ```jwe proc kill <pid>```