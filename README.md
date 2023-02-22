[![Rust](https://github.com/denispeplin/so_notifier/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/denispeplin/so_notifier/actions/workflows/rust.yml)

# Stack Overflow notifier

The app notifies about new questions from Stack Overflow.
It does that querying the Stack Overflow API:
fetch a new question every minute and send a notification
about new questions only.

### The algorithm

Question IDs are incremental, that makes finding new
questions an easy task.
The notification algorithm uses two vectors and one `u32`
number containing the latest question ID that is
stored between runs.

It is meant to be running in a loop and execute the following steps:

- Get latest 10 questions of a particular tag
- Save them into an vector of a freshly retrieved records
- Copy new records (those IDs are greater than the latest
  question ID) into another vector of new questions
    - if the latest question id equals to its initial value of
      `u32::MAX` (means it's the first run), no new questions
      would be collected
- Go through the new questions and send notifications
- Update the value of the latest question ID
- Wait one minute before continuing the loop

### Build

Make sure `libssl-dev` is installed

```sh
# for Ubuntu
sudo apt install libssl-dev
```

Run it from source with Rust installed

```sh
cargo run
```

See also the next section to run the app with an API key

### Install and run

Being in the project's directory, run

```sh
cargo install --path .
```

By default, the binary goes to `.cargo/bin/so_notifier` in your
home directory.

Run it as any other binary with the following command

```sh
~/.cargo/bin/so_notifier
```

Also, don't forget to export the auth key if you want a decent
API request limit (see
[The API and throttle](#the-api-and-throttle)).

```sh
# ~/.profile
export SO_NOTIFY_AUTH_KEY='<the key>'
```

### Logging

The installed version uses `syslog` for logging. By default, the
messages are going to `/var/log/syslog`. You can extract the
messages using `grep`.

For example

```sh
grep so_notifier /var/log/syslog | tail
```

### Autostart

An autostart has to be done after the window system starts, so
use your window manager autostart facilities.
Some sample instructions:

- [Gnome](https://www.simplified.guide/gnome/automatically-run-program-on-startup)

Since the binary is placed in a `hidden` directory, you may need
to copy the path from [Install and run](#install-and-run) section.

### The API and throttle

The Stackexchange API
[applies number of throttles](https://api.stackexchange.com/docs/throttle),
so to use this app in full, follow these instructions:

1. Do not execute the same API query more often than once a minute.
   This app has one minute timeout hardcoded for now. You can increase it,
   but do not decrease.
2. By default, quota number is only 300 requests. That is enough to try, but
   not enough if you want the app to run for some significant time.
   To increase quota, you would need a Stackexchange application key.
   To obtain the key,
   [register your application](https://stackapps.com/apps/oauth/register).
   You can set `OAuth Domain` to `localhost`, and `Client Side OAuth Flow`
   is not needed.
   Once it's registered, get `Key` from the page with the application
   credentials.

### TODO

- Provide Dockerfile
- Move the question tag to command-line parameters
