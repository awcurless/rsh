# rsh

A simple linux command shell written in rust for educational purposes.

Supports:
* Environment variables via `export`
* Resetting the prompt via `setprompt newprompt>`
* Changing the working directory
* Executing arbitrary programs on the user's machine
* Piping via `|`
* Background jobs via `&`. List running background jobs with the `jobs` builtin.
