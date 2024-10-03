# Samnefni

![Release](https://github.com/chino540off/samnefni/actions/workflows/release.yml/badge.svg)

## Introduction
'samnefni' means 'alias' in icelandic.

This tool create aliases like [git aliases](https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases) but for any command.

I always run `docker run -it --rm ` command, but it is too long to write. I would like to write something shorter like `d r`.

## Configuration

Configuration is based on a `toml` file. You can specify path of this command when you call the binary via `--config` option, or by setting the environment variable `SAMNEFNI_CONFIG`. The default path is `~/.samnefni.toml`.

Configuration example:
```toml
[aliases.docker]
r = { args = ["run", "-it", "--rm"] }
e = { args = ["exec", "-it"] }

[aliases.kubectl]
a = { args = ["apply"] }
```

## How to use it

You can call `samnefni` like:
``` sh
samnefni exec docker r python:3
Python 3.12.4 (main, Jul 23 2024, 07:23:10) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>>
```

To make it shorter, you can define an alias for the target command like:
``` sh
alias d="samnefni exec docker --"
d r python:3
Python 3.12.4 (main, Jul 23 2024, 07:23:10) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>>
```
