# Writeup

We have 3 endpoints:
- ```/register?lat=&lon=``` returns a uuid, username, and elo
- ```/match?uuid=&lat=&lon=``` returns an opponent uuid, username, distance, elo
- ```/battle?uuid=&opponent=&number=``` returns a result, new elo, and the opponents number.


There is no authentication being done with the uuid. We can impersonate our opponents as we match with them by taking their uuids, and using them to find other matches until we reach.

Once we are able to match with geopy, we have a very precise distance that we can use to figure out the exact location using the ```/match``` endpoint. We can trilaterate/triangulate the exact location by adjusting the lat/lon parameters in our calls to ```/match```, and once we get to the level of precision we want (we can check our results again by putting it into lat/lon), we find that the location is 1059 S High St, Columbus, OH 43206



# Running

Run `docker-compose up`.
To run in headless mode use `docker-compose up -d`.



