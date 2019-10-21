# RAG: A simple Rust Adventure Game 

## Author List

- kirypto
- Wanooknox


## Project Goals

- Implement a room management system
    - Each room supports a set of commands
    - Initial minimum command support "go" and " look"
- Player should be able to execute commands via text entry
    - "go" command will allow the player to move between directly adjacent rooms (could expand to include portals)
    - "look" command will allow player to access a description of the room they currently occupy
- Rooms shall be encoded into JSON of the form:
```
{
    "<roomId>" : {
        "go" : [ {"Direction":"<roomId>"} ],
        "looks" : "<look falvour text>"
    },
    ...
}
```
