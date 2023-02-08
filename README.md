# Stack Overflow notifier

The app notifies about new questions from Stack Overflow.
It does that querying the Stack Overflow API:
fetch a new question every minute and send a notification
about new questions only.

### The algorithm

The notification algorithm uses one vector and one HashSet:
the vector to store records that were just retrieved and
a HashSet to store question IDs from the previous run between
runs.

It is meant to be running in a loop and execute the following steps:

- Get latest 10 questions of a particular tag
- Save them into an vector of a freshly retrieved records
- Check if the HashSet has some question IDs
    - If it has, send notifications about the new records
    - Otherwise (means it's the first run), go to the next step
- Put the freshly retrieved question IDs into the HashSet
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

- Organize code better and add tests
- Simplify the algorithm for finding new questions
- Provide Dockerfile
- Add Github CI configurationa and the badge
- Handle no-show errors in notify-rust library (stop waiting for it after some timeout)
