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

- Add tests
- Handle more networking errors gracefully
- Provide Dockerfile
- Add Github CI configurationa and the badge
- Add instruction for autostarting the app
- Move the question tag to command-line parameters
