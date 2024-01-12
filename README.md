# ForgeFIX

ForgeFIX is an opinionated FIX 4.2 client library for the buy-side written in Rust.  ForgeFIX is optimized for the subset of the FIX protocol used by buy-side firms connecting to brokers and exchanges for communicating orders and fills.  

# Approach
ForgeFIX is an "opinionated" client.  The primary opinion expressed is that several parts of the FIX protocol are vestigial and/or rarely used.  Please see "Not implemented" below for more information.

## Other opinions embodied in the ForgeFIX code are:
 
 * Safe language -- Mission critical code in production environments should be written in safe languages to minimize crashes, memory corruption and security concerns.  Rust is chosen
for safety as well as robust community and excellend compiler.
 * Minimal allocation -- ForgeFIX endeavors to do as little dynamic allocation as possible on the critical path of sending and receiving messages to maximize throughput and latency.
 * Readable code -- Prioritize readable code over clever code.  Only introduce complexity when required by a desired feature or justified by performance profiling.
 * Useful for live trades -- ForgeFIX is featured enough to carry most live trading traffic, supporting all session management features, message types and fields.
 * On-demand message parsing -- Delay parsing of incoming messages until demanded by client code.

# Features
* FIX 4.2 -- Full message and field support for FIX 4.2.   Session managment, including sequence number negotiation and message resend.
* Database-backed message store -- All valid and processed messages are stored in a local database (Sqlite currently), both to support message resend and offline auditing and querying.
* File logging –- All messages sent, and all received on the wire, whether valid or not, are written a log file for offline auditing.
* Async Rust API -- Async API for Rust code compatible with the Tokio runtime
* C API -- API for use with C code, or through FFI with many others (Python, Go, etc.)
* Testing Suite -- Run multiple test-cases against the ForgeFIX to confirm adherence to FIX 4.2 spec. 

# Status
ForgeFIX is feature complete, and is used in production carrying live orders.  Please consider it--however--to be a beta release until version 1.0 is released.  API changes
are likely to occur prior to 1.0 that will be both forward- and backward- incompatible.


# Not implemented
The following items are not implemented in the interest of simplicity and performance.  The list below was created based on perceived need of buy-side clients connecting to
brokers and exchanges, but is not set in stone.  If you have a use case for one of these features, please open an issue with the "feature request" template.

 * FIX protocol versions other than 4.2 -- Despite there being five revisions of FIX 4.x as well as FIX 5.0 and several other variants, the overwhelming majority of connections
  for communication orders are done with FIX 4.2.
 * In-message encryption -- The FIX protocol allows for message bodies to be encrypted.  In practice this is rarely used.  Much more common is channel encryption (like a VPN), private network links, or both.
 * Message body validation -- A buy side firm is typically sending orders to a counterparty and receiving back 'execution reports'.  A badly formed order message is rejected
 by the counterparty and garners a Reject (3) or ExecutionReport (8), indicating the error.  Assuming an order is well-formed, a broker may respond with an ExecutionReport that is 'malformed' according to the agreed-upon variant of the FIX protocol.   Nothing good comes from rejecting this ExecutionReport.  A buy side firm should do its best to process the execution report, and address the validation issue out-of-band with the counterparty.
 * Automatic reconnection -- One feature of a FIX session is that it can survive multiple tcp disconnections, expected or unexpected. With automatic reconnection, ForgeFIX would be able to handle a TCP disconnection gracefully, without intervention from the client code. However, a TCP disconnection is currently returned as an error in ForgeFIX, and would have to manually reconnect to the FIX session.
