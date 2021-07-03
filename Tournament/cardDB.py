import re
import requests
import json

# Helps with memory being consumed
import gc
import ctypes

from time import time

normaliseRegex = re.compile(",|\.|-|'")
spacesRegex = re.compile(" +")


# Makes two strings easier to compare by removing excess whitespace,
# commas, hyphens, apostrophes and full stops.
def normaliseCardName(string: str):
    return re.sub(spacesRegex, " ", re.sub(normaliseRegex, "", string)).lower().split("//")[0].strip()

class card:
    def __init__(self, name: str, layout: str):
        self.name = name
        if layout in ["modal_dfc", "transform", "flip"]:
            self.name = self.name.split("//")[0]
        self.name = self.name.strip()
    
    def __str__(self):
        return f'{self.name} ({self.layout})'
    
class cardDB:
    def __init__(self, updateTime: int = 24*60*60, mtgjsonURL: str = "https://www.mtgjson.com/api/v5/AllPrintings.json"):
        self.lastUpdate = 0
        self.updateTime = updateTime
        self.cards = dict( )
        self.url = mtgjsonURL
        
        self.updateCards()
    
    def needsUpdate(self) -> bool:
        return int(time()) - self.lastUpdate > self.updateTime
    
    #@profile
    def updateCardsFromJson(self, cardsJson: str) -> bool:
        tempCards = dict( )
        parseSuccess = True
        
        # Try parse, if it goes wrong cry
        cardsJson = json.loads(cardsJson)
        try:
            data = cardsJson["data"]
            for set in data:
                for card_ in data[set]["cards"]:
                    # Check for reprint (also stops the back of a mdfc from being added)
                    # i hate mdfcs as they make this harder than it has to be
                    name = normaliseCardName(card_["name"])
                        
                    if not name in tempCards:
                        cardObject = card(card_["name"], card_["layout"])
                        if "face" in card_:
                            if card_["face"] != "a":
                                continue # Rear of the card is ignored
                            
                        tempCards[name] = cardObject
                                    
        except Exception as e:
            parseSuccess = False
            print(e)
        
        del cardsJson
        
        if parseSuccess:
            self.cards = tempCards
            self.lastUpdate = int(time())
        return parseSuccess
        
    #@profile
    def updateCards(self) -> bool:
        with requests.get(self.url, timeout=7.0, data="",  verify=False) as resp:        
            status: bool = self.updateCardsFromJson(resp.text)
        
        # Try and force python to collect some garbage
        gc.collect()
        libc = ctypes.CDLL("libc.so.6")
        libc.malloc_trim(0)
        
        if status:            
            self.lastUpdate = int(time())
        
        return status

    # Returns a card object from a database search.
    def getCard(self, cardName) -> card:    
        name = ""
        
        if normaliseCardName(cardName) in self.cards:
            name = self.cards[normaliseCardName(cardName)].name
        else:
            name = cardName
        
        print(name)
        return name
