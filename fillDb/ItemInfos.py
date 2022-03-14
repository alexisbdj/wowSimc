import json
from typing import Any
import exception

def getValue(dict, key, fullRaw) -> Any:
    try:
        return dict[key]
    except KeyError:
        raise exception.UnexpectedContentFormat(fullRaw, key)


class ItemClass:
    def __init__(self, data):
        raw = json.dumps(data)
        self.id = getValue(data, 'id', raw)
        self.name = getValue(data, 'name', raw)

class ItemInfos:
    def __init__(self, raw):
        data = json.loads(raw)
        if (type(data) is dict):
            self.id = getValue(data, 'id', raw)
            self.name = getValue(data, 'name', raw)
            self.quality = getValue(getValue(data, 'quality', raw), 'type', raw)
            self.level = getValue(data, 'level', raw)
            self.iclass = ItemClass(getValue(data, 'item_class', raw))
            self.subclass = ItemClass(getValue(data, 'item_subclass', raw))
            self.isEquippable = getValue(data, 'is_equippable', raw)
            self.invType = getValue(getValue(data, 'inventory_type', raw), 'type', raw)

        else:
            raise exception.UnexpectedContentFormat(raw)
        
