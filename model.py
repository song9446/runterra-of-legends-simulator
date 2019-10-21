import lor_deck_encoder

class World:
    def __init__(self):
        self.turns = 0
        self.attack_player
        self.players = []
        self.fields = []
        self.attack = []
        self.defence = []

class CardEventListener:
    def __init__(self):
        pass
    def round_start(self, world):
        pass
    def round_end(self, world):
        pass
    def draw(self, world):
        pass
    def reclaim(self, world):
        pass
    def attack_start(self, world):
        pass
    def attack_start(self, world):
        pass
    def stun(self, world):
    def freeze(self, world):

class CardKeywords(CardEventListener):
    def __init__(self):
        pass
    def active(self, world):
        pass
    def deactive(self, world):
        pass
    def round_start(self, world):
        pass

class CardState:
    def __init__(self):
        pass
    def active(self, world):
        pass
    def deactive(self, world):
        pass

class Card:
    def __init__(self, code):
    def round_start(field, deck):
    def round_end():


class Player:
    def __init__(self, code):
        deck = lor_deck_encode.get_deck_from_code(code)
        self.stack = [card for card in deck]
        self.hands = []
