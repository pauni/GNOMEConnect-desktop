Here are a few features that we want to implement

If you think you can help, please do so.


- [ ] Protocol
  - [ ] Secure device pairing
  - [ ] Extendable
  - [ ] Support Relayserver

- [ ] Features
  - [ ] Notification syncing
  - [ ] Libreoffice impress remote
  - [ ] File sharing
  - [ ] Filesystem mounting
  - [ ] Instant screenshot syncing
  - [ ] Media Control
  - [ ] Remote input (Keyboard, Mouse)





---

# Protocol




Every Package is wrapped in something we calling `Header`.
That's valid for request, responses, everything. Even if a tcp connections is alreadey established.

The reason for this is, that this makes it a lot easier to implement Relays, or brigdes.
For the Relay server the goal is, to decentralize it and let everyone operate an relay server.


## Data standards



### Binary Data
Binary data should be base64 encoded.

maybe we will add some comression standard later


### Encryption
The protocol uses RSA encryption and signing.
(actually we don't use any signing since there is no library for rust).



## Protocol objects


### Header

```json
{
  "src_fingerprint": "fingerprint of the sender",
  "dst_fingerprint": "destination fingerprint",
  "version": 45,
  "payload": {
    "type": "payload_type",
    "data": "actual data"
  }
}
```

#### Fields

##### `src_fingerprint`
Rsa public key fingerprint of the sender

##### `dst_fingerprint`
Rsa public key fingerprint of the receiver. Used for relaying with a relay server.

##### `version`
The protocol version. A receiver can simply check if the senders protocol is compatible or require a minimum version.




### Payload

A payload looks like this:
```json
{
  "type": "Encryped",
  "data": "data"
}
```
or
```json
{
  "type": "Pairing",
  "data": {
    pairing data,
    ...
  }
}
```



### Payload Types

There are at the moment 2 txpes of payloads
- `Pairing`
- `Encrypted`





### Pairing




| Handy                        | Desktop                    |
| :--------------------------- | -------------------------: |
| Hello, i wanna pair with you |                            |
|                              | Ok, lets share our keys    |
| {public key}                 |                            |
|                              | {public key}               |
| E N C R Y P T E D _ N O W    | E N C R Y P T E D _ N O W  |
| I am {name}, wanna marry me? |                            |
|                              | Yes, i want                |
