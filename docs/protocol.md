# Backupd protocol

Backupd uses [bincode](https://github.com/servo/bincode) to serialize and
transport data. All the structs that get sent on the wire are defined in the
`backupd::protocol` module.
