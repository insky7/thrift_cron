# python-cron-replies

A Rust cron job that closes the loop on customer replies for **Promise**, a customer-communication platform used by automotive service shops, body shops, dealers, and insurers to keep customers updated on repair/appointment status by SMS and email. Customers reply to those messages in all the usual messy ways (`STOP`, `HELP`, "yes", forwarded threads, quoted replies, bounces...), and this service is what reads those replies and turns them back into actions inside the platform.

It is a Rust rewrite of an older Python cron job (hence the crate name `python-cron-replies`), built to sit next to a much larger, older PHP monolith. The rewrite intentionally preserves the original's behavior and quirks rather than redesigning the flow this is a port, not a rearchitecture.

## Where this fits

The PHP backend owns all of the actual business data (`Promise`, `Customer`, `Company`, `User`, ...) and is the system everyone else talks to. This cron job is a satellite process with three jobs:

1. **Collect** inbound replies from two different places an IMAP mailbox (email replies) and a MySQL queue table (SMS replies relayed in from a carrier/gateway upstream of this service).
2. **Classify** each reply against the message body and matches it back to the `Promise` record it belongs to.
3. **Act** on it by calling back into the PHP backend over Thrift RPC logging the event, forwarding messages, updating flags, or sending a canned reply and by kicking off a couple of read-only side effects (real-time push notification, sentiment analysis).

It does not own any data itself; MySQL access is read/write against the *same* database the PHP app uses, and all "real" actions funnel back through the legacy backend's Thrift services. That makes this service closer to an ETL/orchestration layer than a standalone application its output is Thrift calls and SQL writes into someone else's schema.

```
                 ┌──────────────┐        ┌──────────────────────┐
  IMAP mailbox   │              │        │                      │
  (email replies)├──▶  this cron ├───────▶│  PHP backend (Thrift)│
                 │   job        │        │  Promise / Alert /    │
  MySQL          │  (Rust,      │◀──────▶│  Messaging / Transcript
  MessageQueue   │   scheduled) │  MySQL │  services, same DB    │
  (SMS replies)  │              │        │                      │
                 └──────┬───────┘        └──────────────────────┘
                        │
                        ├──▶ STOMP "pusher" queue (real-time UI notifications)
                        └──▶ sentiment-analysis HTTP endpoint
```

## How a reply is processed

Each run is a single pass, not a long-lived daemon it's meant to be invoked on a schedule (hence "cron"):

1. **Fetch.** Query `MessageQueue` for unprocessed inbound SMS (`iProcessedFlag = 0 AND iFromMobileFlag = 1`), and fetch every message currently sitting in the IMAP `INBOX`. Both get parsed into a common `NotifierReply` enum (`SMS` or `Email` see `types.rs`) so the rest of the pipeline doesn't care which channel a reply came in on. Bounce/delivery-failure emails are detected and dropped here.

2. **Match.** Each reply is matched back to a `Promise` row by an embedded promise ID (email), or by mobile number / from-address plus a same-industry/company join (SMS), see `find_promise` in `db_interactions.rs`.

3. **Classify.** The message body is scanned line-by-line for a small set of keywords/patterns, checked in this order:
   - a STOP-phrase check against the Thrift `TcpaService` (compliance always checked first, regardless of anything else)
   - `HELP`, `TNEWS`, `VEHICLE` fixed keyword commands
   - `C` opt-in confirmation, only meaningful in an SMS context
   - an embedded `Promise ID: [...]` token marks the reply as a forwarded "estimator reply" thread
   - anything else falls through to the default customer-reply handler

4. **Act**, per classification (see `types.rs` for the full implementation):
   - **STOP** opts the customer out via `TcpaService`, deactivates any matching `TestDrive`/`Promise` record, logs a `PromiseEventLog` entry, and notifies the estimator; if no promise matches, sends a generic opt-out confirmation instead.
   - **HELP** / **TNEWS** / **VEHICLE** look up a canned system message (`SysConfig`) and send it back over SMS. `VEHICLE` sends two messages with a deliberate 1s gap between them (preserved from the Python original).
   - **ESTREPLY** decodes the embedded promise/login/customer identifiers from the `Promise ID: [...]` token and forwards the message either to the estimator or, for a 3rd-party CC'd reply, back to whoever it was addressed to.
   - **OPTIN** flips the promise's double opt-in flag if one is pending.
   - **default (plain customer reply)** this is the richest path: logs the event, strips known survey/short-link text and profanity (`bad_words.rs`) from the stored copy, records a `Transcript`, notifies the estimator/insurance contact/local users depending on who the message was addressed to, fires an away-message auto-responder if the assigned user is marked away (rate-limited to one per `AUTORESPOND_INTERVAL_HOURS`), enqueues the message for sentiment scoring, and pushes a STOMP notification so the web UI updates live. For autobody-industry promises specifically, it additionally checks whether the reply is a "yes"/"no" answer to an outstanding repair-authorization request and, if so, logs that as a distinct authorization decision instead of a generic reply.

5. **Mark done.** Processed SMS rows get `iProcessedFlag = 1`. (Processed emails aren't similarly marked every run re-reads the whole IMAP inbox; whatever happens to email state/archiving lives outside this codebase.)

## Talking to the legacy backend

The PHP backend exposes its API as a set of Thrift services multiplexed over a single TCP connection (`TMultiplexedOutputProtocol`, see `thrift/rpc.rs`). `ThriftClientContext` (`thrift/mod.rs`) opens one connection and hands back typed sync clients for the services this job actually needs: `System` (config values), `Tcpa` (compliance/opt-out), `Messaging` (send SMS/email), `Transcript` (save conversation history), `Alert`/`AlertMessage` (notify users), and `Promise` (the core promise-mutation API `stop`, `send_customer_message`, `get_tokens`, `replace_tokens`, ...).

`src/thrift/autogen.rs` is generated from `rpc.thrift` at the repo root that file is the actual IDL contract shared with the PHP side and covers far more services than this crate uses (registration, scheduling, DMS integrations, VPP products, etc.); this crate only wires up the handful of clients it calls.

## Project layout

```
src/
  main.rs               Entry point: loads .env, opens the DB pool + Thrift connection, runs process_replies()
  db_interactions.rs    Promise matching (find_promise) + process_replies() SMS/email fan-in
  imap_interactions.rs  IMAP fetch + email parsing/normalization into NotifierEmailV2
  types.rs              NotifierReply/NotifierEmail/NotifierSMS + all reply-classification/handling logic
  helper_functions.rs   Logging setup, phone/email formatting, STOMP (pusher) notifications, sentiment queue, misc SQL helpers
  bad_words.rs          Loads a profanity list from the DB and censors matched words in outgoing/stored messages
  py_types.rs           Notes ported from the old Python/PHP models, mostly commented out (reference only)
  thrift/
    mod.rs              ThriftClientContext: opens one TCP connection, wires up all the typed service clients
    rpc.rs              Multiplexed-protocol plumbing shared by every client
    autogen.rs           Generated Thrift bindings for the full rpc.thrift IDL (do not hand-edit)
rpc.thrift              Thrift IDL shared with the PHP backend source of truth for autogen.rs
test_email.py           Standalone script for manually sending a test email via SMTP against a live mailbox
```

## Requirements

- Rust (2024 edition toolchain)
- A reachable MySQL database matching the legacy `Promise`/`Customer`/`Login`/`Company`/`Industry`/`MessageQueue`/`PromiseEventLog`/... schema `sqlx`'s query macros are compile-time checked against `DATABASE_URL`, so a real schema has to be reachable at build time, not just at runtime
- Network access to the legacy PHP backend's Thrift RPC server
- An IMAP mailbox to poll (plaintext, port 143)
- A STOMP endpoint ("pusher") for real-time UI notifications
- A sentiment-analysis HTTP endpoint

## Configuration

Configuration is read from a `.env` file (via `dotenvy`; the file is git-ignored never commit it) plus process environment:

| Variable | Purpose |
|---|---|
| `DATABASE_URL` | MySQL connection string |
| `IMAP_SERVER` | IMAP host to poll for email replies |
| `IMAP_USER` / `IMAP_PASS` | IMAP credentials |
| `MailSendersInsurance` | Email address used to detect insurance-directed replies |
| `ShortCodesInsurance` | SMS short code used to detect insurance-directed replies |
| `PusherHost` / `PusherPort` | STOMP endpoint for real-time notifications |
| `SentimentsHost` | Host for the sentiment-analysis cache-queue endpoint |

## Building & running

```sh
cargo build
cargo run
```

Each run does a single fetch-classify-act pass over whatever's currently pending in IMAP and `MessageQueue`, then exits schedule it externally (cron, a task scheduler, etc.) for repeated polling. Logs are written to daily-rotated files (`prefix.log.<date>`) in the working directory via `tracing-appender`.

## Tests

```sh
cargo test
```

Unit tests cover the `NotifierReply`/`NotifierEmail`/`NotifierSMS` accessor and serialization logic in `types.rs`.
