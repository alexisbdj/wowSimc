from ItemInfos import *
import mysql.connector


def getItemQuery(item: ItemInfos):
    return "INSERT INTO Item (id, name, level, isEquippable, itemClass, itemSubClass, invType) VALUES ({id}, \"{name}\", {level}, {isEquippable}, {iclass}, {subClass}, \"{invType}\")".format(
        id=item.id,
        name=item.name,
        level=item.level,
        isEquippable=item.isEquippable,
        iclass=item.iclass.id,
        subClass=item.subclass.id,
        invType=item.invType,
    )


def linkItemToDungeon(item: ItemInfos, dgName: str):
    return "INSERT INTO SourceDrop (dungeonName, itemId) VALUES (\"{dgName}\", {itemId})".format(
        dgName=dgName,
        itemId=item.id
    )


def processItem(item: ItemInfos, dungeonName: str, cnx: mysql.connector.connection_cext.CMySQLConnection):
    cursor = cnx.cursor()
    cursor.execute(getItemQuery(item))
    cursor.execute(linkItemToDungeon(item, dungeonName))
    cnx.commit()
