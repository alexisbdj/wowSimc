import os
import apiCall
from ItemInfos import ItemInfos
import processItem

def getCacheFilePath(itemId) -> str:
    return './cache/{itemId}.json'.format(itemId=itemId)

def getItemFromApi(api: apiCall.Api, itemId) -> str:
    r = api.getItem(itemId)
    if (r.status_code == 200):
        with open(getCacheFilePath(itemId), 'w') as file:
            file.write(r.text)
        return r.text
    else:
        print("erreur")

def processItemEntry(dungeonName, itemId, itemName, api: apiCall.Api, cnx):
    raw = None
    if (os.path.exists(getCacheFilePath(itemId))):
        with open(getCacheFilePath(itemId)) as file:
            raw = file.read()
    else:
        raw = getItemFromApi(api, itemId)
        print(itemId, "cached")
    processItem.processItem(ItemInfos(raw), dungeonName, cnx)

def dungeonExistsImNowAnswering(dungeonName, cnx):
    cursor = cnx.cursor(buffered=True)
    cursor.execute('SELECT * FROM Dungeon WHERE name = "{dungeonName}"'.format(dungeonName=dungeonName))
    if (cursor.rowcount == 0):
        cursor.execute('INSERT INTO Dungeon (name) VALUES ("{}")'.format(dungeonName))
    cnx.commit()

def run(dungeonName, cnx):
    dungeonExistsImNowAnswering(dungeonName, cnx)
    api = apiCall.Api()
    with open('./mplusitemsList/{dungeonName}.txt'.format(dungeonName=dungeonName)) as file:
        for line in file:
            line = str.rstrip(line)
            split = line.split(' ', 1)
            itemId = int(split[0])
            itemName = split[1]
            processItemEntry(dungeonName, itemId, itemName, api, cnx)