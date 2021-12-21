### Install
#### from source

_Requires `rust` toolchain. see https://rustup.rs/_

- checkout source
- cd to repo root
- run `cargo install -p <cli>`

This will install all binaries to `$HOME/.cargo/bin` (make sure that it's on your `$PATH`)

### Configuration

All tools are based on the same libraries, and they all share some configuration entries.

e.g. when configuring aws related options they will apply to all tools (unless you specify in a tool-specific config file)

#### Profiles

The config system is profile based.

The `default` profile is loaded first. Then the __selected profile__ is loaded. Finally the `global` profile is loaded
and overwrites any previous values.

The __selected profile__ is determined either using the CLI switch `-p <profile>`/`--profile <profile>` or by reading
the contents of `~/.cd/profile`.

#### Files

The file format is TOML. To allow for flexible configuration multiple files are loaded depending on the `profile` and the tool `name`.

- `~/.cd/default.conf`
- `~/.cd/<profile>.conf`
- `~/.cd/<name>.conf`
- `~/.cd/global.conf`

If the file name contains a profile name (`default`, `global`, ...), then values are loaded directly into that profile.

`default.conf`
```toml
[foo]
bar = "hello config"
```

Effectively configures `foo.bar` to value `hello config`.

The exception is the `<name>.conf` file; the first configuration level is the profile name, so `<key...>` becomes `<profile>.<key...>`.
To configure `foo.bar` to value `hello config` in the `default` profile you would write:

`<name>.conf`
```toml
[default.foo]
bar = "hello config"
```
or 
```toml
[default]
foo.bar = "hello config"
```
