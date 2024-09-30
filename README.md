# Changelog manager

This project aims to provide a CLI tool to manage CHANGELOG entries, in order to avoid the infamous [changelog conflict crisis](https://about.gitlab.com/blog/2018/07/03/solving-gitlabs-changelog-conflict-crisis/) (and, I cannot deny it, a side project to play with Rust).

## Missing features

- [ ] update CLI from Gitlab repository
- [ ] interactive mode
- [ ] manage a config from multiple sources (home directory, current directory, environment variables, CLI ?)
  - username
  - Gitlab's token to fetch issues in interactive mode, and validate the issue's existence in static mode
- [ ] template to add entries in a specific way
- [ ] dynamic fields support (to allow users to add specific fields in each entry - needs templates implementation)
