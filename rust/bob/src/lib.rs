pub fn reply(message: &str) -> &str {
    // unimplemented!("have Bob reply to the incoming message: {}", message)
    const QUESTION: &str = "Sure.";
    const YELL: &str = "Whoa, chill out!";
    const YELL_QUESTION: &str = "Calm down, I know what I'm doing!";
    const NOTHING: &str = "Fine. Be that way!";
    const ELSE: &str = "Whatever.";

    let msg = message.trim();
    if msg.len() == 0 {
        NOTHING
    } else {
        let mut upper = false;
        let mut quest = false;
        if msg == msg.to_uppercase() && msg != msg.to_lowercase() {
            upper = true;
        }
        if msg.as_bytes()[(msg.len() - 1)] == b'?' {
            quest = true;
        }

        if upper && quest {
            YELL_QUESTION
        } else if upper {
            YELL
        } else if quest {
            QUESTION
        } else {
            ELSE
        }
    }
}
