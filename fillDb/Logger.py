import os
import exception

class Singleton(type):
    _instances = {}
    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]

class Logger(metaclass=Singleton):
    def __init__(self):
        logPath = os.environ.get('LOG_PATH')
        if (logPath == None):
            self.file = None
        else:
            self.file = open(logPath, "a")

    def Log(self, message: str):
        if (self.file == None):
            return
        self.file.write(message)
