from email import header
import json
import requests
import os
from requests.auth import HTTPBasicAuth
import exception

class Api:

    def getAnswer(self, response, contentDescription):
        try:
            return response.json()
        except json.decoder.JSONDecodeError:
            print("failed to parse: ", contentDescription)
            raise exception.UnexpectedContentFormat(response.text)

    def getToken(self):
        if (self.token != None):
            return self.token
        r = requests.post(url="https://{region}.battle.net/oauth/token".format(region=self.region), auth=HTTPBasicAuth(self.clientId, self.secret), data={'grant_type': 'client_credentials'})
        if (r.status_code != 200):
            print(r.status_code)
            raise exception.Unauthorized
        result = self.getAnswer(r, "access_token")
        try:
            access_token = result['access_token']
            self.token = access_token
            return access_token
        except KeyError:
            raise exception.UnexpectedContentFormat(json.dumps(result), 'access_token')

    def makeUri(self, route):
        return "https://{region}.api.blizzard.com{route}".format(region="eu", route=route)

    def call(self, route, params):
        print(route)
        token = self.getToken()
        return requests.get(url=self.makeUri(route), params=params, headers={'Authorization': 'Bearer {access_token}'.format(access_token=token)})

    def __init__(self):
        self.region = "eu"
        self.clientId = os.environ.get("CLIENT_ID")
        self.secret = os.environ.get("CLIENT_SECRET")
        self.token = None
        

    def getItem(self, itemId):
        return self.call('/data/wow/item/{itemId}'.format(itemId=itemId), params={'namespace': 'static-eu', 'locale': 'en_US'})