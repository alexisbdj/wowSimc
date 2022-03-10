class AppException(Exception):
    pass

class NoSqlConnection(AppException):
    pass

class UnexpectedContentFormat(AppException):
    def __init__(self, data=None, missingKey=None):
        if (data == None):
            self.data = "undefined"
        else:
            self.data = data
            self.missingKey = missingKey

    def __str__(self):
        result = "undefined"
        if (self.data != None):
            result = self.data
            if (self.missingKey != None):
                result = "(missing key: {}) {}".format(self.missingKey, result)


class Unauthorized(AppException):
    pass