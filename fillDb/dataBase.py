import string
import mysql.connector
import exception

def connectToDb(user: str, password: str, host: str, database: str) -> mysql.connector.connection_cext.CMySQLConnection:
    try:
        cnx = mysql.connector.connect(user=user, password=password, host=host, database=database)
    except mysql.connector.Error as err:
        raise exception.NoSqlConnection
    else:
        if (isinstance(cnx, mysql.connector.connection_cext.CMySQLConnection)):
            return cnx
        else:
            raise exception.NoSqlConnection