# CircleCI Config

**NOTE: The config in this directory needs to be compiled from source manually when you make changes.**

This directory contains the CircleCI config – both the source and compiled versions:

* Source – `config.src.yml` uses version the 2.1 configuration spec. This is the one we edit.
* Compiled – `config.yml` uses version the 2.0 configuration spec. This is what CircleCI uses.

You should make changes in `config.src.yml` and then compile them on your local machine before committing.

## Setup

On macOS you can install the CircleCI CLI tool using Homebrew:

```sh
brew install circleci
```

For other operating systems, see the [CircleCI docs](https://circleci.com/docs/2.0/local-cli/#installation).

## How to compile `config.src.yml`

To compile the `config.src.yml` to `config.yml`, run the following from the project root directory:

```sh
circleci config process .circleci/config.src.yml > .circleci/config.yml
```

## Why?

CircleCI Enterprise does not support CircleCI 2.1 configuration. In order to use CircleCI 2.1 features such as [reusable config](https://circleci.com/docs/2.0/reusing-config), we need to compile our config file into CircleCI 2.0 before committing it into the repository.
