import base64

vmess_url ="vmess://ew0KICAidiI6ICIyIiwNCiAgInBzIjogIuaXpeacrDAxfOe9kemhteiKgueCuXxYMiIsDQogICJhZGQiOiAianAwMS5udHRray5jb20iLA0KICAicG9ydCI6ICI0NDMiLA0KICAiaWQiOiAiZGIzZmYyZDItNWRkZC0zZWJlLTg1MzgtOTM3YTQ4MGRkMmU2IiwNCiAgImFpZCI6ICIyIiwNCiAgInNjeSI6ICJhdXRvIiwNCiAgIm5ldCI6ICJ3cyIsDQogICJ0eXBlIjogIm5vbmUiLA0KICAiaG9zdCI6ICJqcDAxLnNzdHRray5jb20iLA0KICAicGF0aCI6ICIvdjJyYXkiLA0KICAidGxzIjogInRscyIsDQogICJzbmkiOiAiIiwNCiAgImFscG4iOiAiIg0KfQ=="

decoded_string = base64.b64decode(vmess_url[8:]).decode('utf-8')
print(decoded_string)


