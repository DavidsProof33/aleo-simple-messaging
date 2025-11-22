# Message Format — Aleo Simple Messaging

This document describes the structure of the `Message` record used in the Leo program and how the Rust CLI provides input parameters to the transition.

---

## 1. `Message` Record Structure

Defined in `main.leo`:

```rust
record Message {
    owner: address,
    msg_id: field,
    data0: field,
    data1: field,
    data2: field,
}
```

### Field Description

| Field  | Type    | Description                           |
|--------|---------|---------------------------------------|
| owner  | address | The owner of the private record       |
| msg_id | field   | A message identifier                  |
| data0  | field   | Arbitrary field value                 |
| data1  | field   | Arbitrary field value                 |
| data2  | field   | Arbitrary field value                 |

This structure is intentionally minimal and educational.

---

## 2. Transition Input Format

The Rust CLI invokes:

```
send_message(
    sender: address,
    recipient: address,
    msg_id: field,
    data0: field,
    data1: field,
    data2: field,
)
```

### Example Input

```
sender     = aleo1sender...
recipient  = aleo1recipient...
msg_id     = 1field
data0      = 10field
data1      = 20field
data2      = 30field
```

---

## 3. Record Construction in the Transition

```rust
return Message {
    owner: recipient,
    msg_id: msg_id,
    data0: data0,
    data1: data1,
    data2: data2,
};
```

➡️ The `owner` is always set to the `recipient`.

This follows a standard Leo pattern for private data storage.

---

## 4. Potential Extensions

The schema can be extended easily:

- timestamps  
- sequence numbers / nonces  
- checksums or hashes  
- additional fields  
- multiple record types  

---

## 5. Summary

The message format is minimal by design.  
Its purpose is to illustrate:

- how transition inputs map to Leo record fields,  
- how private records are constructed,  
- how Rust and Leo components interoperate.
