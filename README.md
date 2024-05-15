# Rusty CRIB

Rusty CRIB (Cryptographic Root Identity Bunker) is a web interface for managing your NIP-46 bunker. 
Built on Rust for maximum security and performance, and PWA support for most native devices. 

Design goal is to have a simple, secure and easy to use interface for managing your NIP-46 bunker.

## Features

- [x] Create a new bunker with a private key
- [x] Import an existing bunker with a private key
- [x] Export a NostroBunker 
- [x] Relay selection and configuration
- [ ] Manage multiple keys

### NIP-46 Features 

Currently available:

- `sign_note` 
- `ping`

## Security Tradeoffs

Private key is stored on IndexedDB. This gives security from most web attacks, but 
is still vulnerable to malware and physical access. These will have to be mitigated by the user.
