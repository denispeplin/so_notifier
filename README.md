# Stack Overflow notifier

The app notifies about new questions from Stack Overflow.
It does that by abusing the Stack Overflow API a little:
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
