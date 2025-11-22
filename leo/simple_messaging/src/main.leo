program simple_messaging.aleo {

    // Egyszerű Message rekord – demo célra.
    // Három tetszőleges field adatot tárol.
    record Message {
        owner: address,   // a címzett, aki birtokolja a rekordot
        sender: address,  // a feladó címe
        msg_id: field,    // üzenet azonosító
        data0: field,     // tetszőleges adat 1
        data1: field,     // tetszőleges adat 2
        data2: field,     // tetszőleges adat 3
    }

    /// Egyszerű "üzenetküldés" demo:
    /// - input: sender, recipient, msg_id + 3 field adat
    /// - output: egy Message rekord
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
