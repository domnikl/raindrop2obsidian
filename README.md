# raindrop2obsidian

A highly opinionated tool to automate transferring highlights from [raindrop.io](https://raindrop.io/) to a directory full of Markdown format compatible with Obsidian.

## What you'll need

Go to https://app.raindrop.io/settings/integrations and create a new app. Generate a test token for it and set an environment variable `RAINDROP_ACCESS_TOKEN` for it. You can also use a `.env` file to provide it:

```shell
RAINDROP_ACCESS_TOKEN=xxx
```

## Usage

```
Usage: raindrop2obsidian [OPTIONS] --output-path <OUTPUT_PATH>

Options:
  -o, --output-path <OUTPUT_PATH>  Output directory, will be created if it doesn't exist
      --tag <TAG>                  Additional tags to add to the output
      --overwrite                  Overwrite existing files with the same name
  -h, --help                       Print help
  -V, --version                    Print version
```
