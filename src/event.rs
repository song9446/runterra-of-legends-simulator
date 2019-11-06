trait Expression {
    fn parse(
}

enum Condition {
    RoundStart,
    RoundEnd,
}

trait EventListener {
    fn listen(&self, Events) -> Events;
}

struct Event {
}
