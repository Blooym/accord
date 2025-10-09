# Accord

> [!CAUTION]  
> **This project is made for me, my needs, and my infrastructure.**
>
> No support will be offered for this software. Breaking changes to functionalty or features may be made any time.

A simple multi-server starboard discord application.

## Setup

### Docker

1. Copy [compose.yml](./compose.yml) to a local file named `compose.yml` or add the
   service to your existing stack and fill in the environment variables.
   Information about configuration options can be found in the
   [configuration](#configuration) section.

2. Start the stack

```
docker compose up -d
```

### Manual

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed and
   in your `$PATH`.
2. Install the project binary

```
cargo install --git https://codeberg.org/Blooym/accord.git
```

3. Set configuration values as necessary.
   Information about configuration options can be found in the
   [configuration](#configuration) section.

```
accord
```

## Configuration

Accord is configured via command-line flags or environment variables and has full support for loading from `.env` files. Below is a list of all supported configuration options. You can also run `accord --help` to get an up-to-date including default values.


| Name                 | Description                                                    | Flag                                            | Env                         | Default |
| -------------------- | -------------------------------------------------------------- | ----------------------------------------------- | --------------------------- | ------- |
| Database URL         | SQLite database connection string to use for persisted data    | `--database-url <DATABASE_URL>`                 | `DATABASE_URL`              | -       |
| Discord Token        | The Discord bot token to authenticate with                     | `--discord-token <DISCORD_TOKEN>`               | `ACCORD_DISCORD_TOKEN`      | -       |
| Discord Bot Status   | The custom status to use for the bot's profile                 | `--discord-bot-status <DISCORD_BOT_STATUS>`     | `ACCORD_DISCORD_BOT_STATUS` | -       |
| Discord Dev Guild ID | The guild to register commands for testing (debug builds only) | `--discord-dev-guild-id <DISCORD_DEV_GUILD_ID>` | `ACCORD_DEV_GUILD_ID`       | -       |

## Usage

Use Discord's built in `/` command help menu to learn more about commands and their options. Permissions for commands can be configured via Discord's "integrations" server settings tab.