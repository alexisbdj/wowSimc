#!/bin/python

import os
import dataBase
from dotenv import load_dotenv
import exception
from Logger import Logger
from ItemInfos import *
import sys
import processItem
import traceback
import processDungeon

def getSqlCnx():
    user = os.environ.get('DB_USERNAME')
    password = os.environ.get('DB_PASSWORD')
    host = os.environ.get('DB_HOST')
    database = os.environ.get('DB_NAME')

    if (host == None):
        host = 'localhost'
    return dataBase.connectToDb(user, password, host, database)

def main():
    load_dotenv()
    try:
        with getSqlCnx() as cnx:
            try:
                for dungeonName in [
                    'HallOfAtonement',
                    'Mists',
                    'Necrotic',
                    'OtherSide',
                    'Plaguefall',
                    'Sanguine',
                    'Spires',
                    'Tazavesh',
                    'Theater'
                ]:
                    processDungeon.run(dungeonName, cnx)
            except exception.UnexpectedContentFormat as err:
                Logger().Log(str(err))
                sys.exit(1)
    except exception.NoSqlConnection:
        print("couldn't connect to server")
        Logger().Log(traceback.format_exc())
        sys.exit(1)



if (__name__ == "__main__"):
    try:
        main()
    except exception.AppException:
        print("error occured, logged")
        Logger().Log(traceback.format_exc())
        sys.exit(1)