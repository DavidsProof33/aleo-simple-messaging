program simple_messaging.aleo {

    // Simple Message record – for demo purposes.
    // Stores three arbitrary field values.
    record Message {
        owner: address,   // the recipient address (record owner)
        sender: address,  // the sender address
        msg_id: field,    // message identifier
        data0: field,     // arbitrary data 1
        data1: field,     // arbitrary data 2
        data2: field,     // arbitrary data 3
    }

    /// Simple "send message" demo:
    /// - input: sender, recipient, msg_id + 3 field values
    /// - output: a single Message record
    transition send_message(
        sender: address,
        recipient: address,
        msg_id: field,
        data0: field,
        data1: field,
        data2: field
    ) -> Message {
        let msg = Message {
            owner:  recipient,
            sender: sender,
            msg_id: msg_id,
            data0:  data0,
            data1:  data1,
            data2:  data2,
        };

        return msg;
    }
}
