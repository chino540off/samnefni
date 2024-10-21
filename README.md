# Samnefni

![Release](https://github.com/chino540off/samnefni/actions/workflows/release.yml/badge.svg)

## Introduction
'samnefni' means 'alias' in icelandic.

This tool creates aliases like [git aliases](https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases) but for any command.

I always run `docker run -it --rm ` command, but it is too long to write. I would like to write something shorter like `d r`.

## Configuration

Configuration is based on a `toml` file. You can specify path of this command when you call the binary via `--config` option, or by setting the environment variable `SAMNEFNI_CONFIG`. The default path is `~/.samnefni.toml`.

Configuration example:
```toml
[aliases.docker]
r = "run -it --rm"

[aliases.kubectl]
a = "apply -f..."
```

`[aliases.docker]` defines an alias called `r` with "simple" options and will be expanded with `run -it --rm`.
If you call `samnefni exec docker r arg1 arg2 arg3`, the command line will be
`docker run -it --rm arg1 arg2 arg3`.

`[aliases.kubectl]` defines an alias called `a` with a "fold" option `-f`. Expansion will take extra arguments
and prefix with the specified option. If you call `samnefni exec kubectl a arg1 arg2 arg3`, the command line
will be `kubectl apply -f arg1 -f arg2 -f arg3`.


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

## Examples
### Simple replacement
```toml
[aliases.docker]
# list images
il = "image ls"
```

```sh
alias d='samnefni exec docker --'
RUST_LOG=debug d il
[2024-10-21T09:18:58Z DEBUG samnefni::command] docker il -> docker image ls
[2024-10-21T09:18:58Z INFO  samnefni::command] execute docker image ls
REPOSITORY                                 TAG                            IMAGE ID       CREATED         SIZE
...
```

### Fold expressions
```toml
[aliases.kubectl]
# apply multiple files
a = "apply -f..."
```

```sh
alias k='samnefni exec kubectl --'
RUST_LOG=debug k a $(find manifests -name '*.yaml')
[2024-10-21T09:25:02Z DEBUG samnefni::command] 'kubectl a manifests/service.yaml manifests/ingress.yaml manifests/deployment.yaml manifests/serviceaccount.yaml' -> 'kubectl apply -f manifests/service.yaml -f manifests/ingress.yaml -f manifests/deployment.yaml -f manifests/serviceaccount.yaml'
[2024-10-21T09:25:02Z INFO  samnefni::command] execute 'kubectl apply -f manifests/service.yaml -f manifests/ingress.yaml -f manifests/deployment.yaml -f manifests/serviceaccount.yaml'
...
```

### Shell expressions
```toml
[aliases.docker]
# image remove with pattern
ir = "image rm $(docker image ls | grep ... | tr -s ' ' | cut -d' ' -f 3)"
```

```sh
alias d='samnefni exec docker --'
RUST_LOG=debug d c python
[2024-10-21T09:29:23Z DEBUG samnefni::command] 'docker ir python' -> 'docker image rm $(docker image ls | grep python | tr -s ' ' | cut -d' ' -f 3)'
[2024-10-21T09:29:23Z INFO  samnefni::command] execute 'docker image rm $(docker image ls | grep python | tr -s ' ' | cut -d' ' -f 3)'
Untagged: python:3
...
```
