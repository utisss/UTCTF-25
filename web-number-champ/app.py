from uuid import uuid4
from flask import Flask, request, jsonify, render_template
from math import radians, sin, cos, sqrt, atan2
import random
from geopy import distance

app = Flask(__name__)

users = {}

def generate_name():
    animals = ["Dog", "Cat", "Bird", "Fish", "Turtle", "Rabbit", "Horse", "Pig", "Cow", "Sheep", "Goat", "Chicken", "Duck", "Goose", "Turkey", "Pigeon", "Parrot", "Canary", "Ostrich", "Peacock", "Penguin", "Seagull", "Eagle", "Hawk", "Falcon", "Owl", "Raven", "Crow", "Sparrow", "Robin", "Bluejay", "Cardinal", "Woodpecker", "Hummingbird", "Swan", "Crane", "Flamingo", "Pelican", "Stork", "Heron", "Albatross", "Puffin", "Kiwi", "Emu", "Cassowary", "Rhea", "Vulture", "Condor", "Buzzard", "Osprey", "Kite", "Harrier", "Kestrel", "Merlin", "Goshawk", "Sparrowhawk", "Hobby", "Peregrine", "Lanner", "Saker", "Gyr", "Barbary", "Falconet", "Caracara", "Secretary", "Bird", "Seriemas", "Sunbittern", "Kagu", "Bustard", "Jacana", "Painted", "Snipe", "Woodcock", "Sandpiper", "Curlew", "Godwit", "Stilt", "Avocet", "Oystercatcher", "Plover", "Lapwing", "Pratincole", "Skimmer", "Skuas", "Gull", "Tern", "Noddy", "Skua", "Jaeger", "Shearwater", "Petrel", "Albatross", "Fulmar", "Storm", "Pelican", "Booby", "Cormorant", "Frigatebird", "Anhinga", "Darter", "Gannet", "Penguin", "Auk", "Puffin", "Guillemot", "Murre", "Razorbill", "Dove", "Pigeon", "Cuckoo", "Nightjar", "Swift", "Kingfisher", "Bee-eater", "Roller", "Hoopoe", "Woodpecker", "Wryneck", "Piculet", "Lark", "Swallow", "House", "Martin", "Pipit", "Wagtail", "Cuckoo-shrike", "Bulbul", "Thrush", "Chiffchaff", "Warbler", "Flycatch"]
    adjectives = ["Red", "Blue", "Green", "Yellow", "Orange", "Purple", "Black", "White", "Grey", "Brown", "Pink", "Gold", "Silver", "Copper", "Bronze", "Iron", "Steel", "Titanium", "Platinum", "Diamond", "Ruby", "Sapphire", "Emerald", "Amethyst", "Topaz", "Pearl", "Opal", "Jade", "Onyx", "Obsidian", "Crystal", "Quartz", "Granite", "Marble", "Limestone", "Sandstone", "Basalt", "Gabbro", "Diorite", "Andesite", "Rhyolite", "Travertine", "Slate", "Shale", "Gneiss", "Schist", "Phyllite", "Mica", "Feldspar", "Quartzite", "Lapis", "Lazuli", "Malachite", "Turquoise", "Coral", "Ivory", "Bone", "Horn", "Tusk", "Tooth", "Claw", "Fang", "Scale", "Feather", "Fur", "Hair", "Wool", "Silk", "Cotton", "Linen", "Leather", "Rubber", "Plastic", "Glass", "Metal", "Wood", "Stone", "Clay", "Sand", "Dirt", "Mud", "Gravel", "Pebble", "Rock", "Boulder", "Hill", "Mountain", "Valley", "Canyon", "Plateau", "Plain", "Desert", "Tundra", "Taiga", "Forest", "Jungle", "Swamp", "Marsh", "Wetland", "River", "Lake", "Pond", "Stream", "Creek", "Brook", "Spring", "Well", "Puddle", "Pool", "Ocean", "Sea", "Gulf", "Bay", "Strait", "Channel", "Sound", "Fjord", "Estuary", "Delta", "Island", "Peninsula", "Isthmus", "Cape", "Point", "Beach", "Shore", "Coast", "Cliff", "Cave", "Cavern", "Grotto", "Lagoon", "Atoll", "Reef", "Shoal", "Sandbar", "Spit", "Dune", "Bluff", "Mesa", "Butte", "Monument", "Arch", "Bridge", "Tower", "Castle", "Fort", "Citadel"]
    return random.choice(adjectives) + " " + random.choice(animals) + " " + str(random.randint(1, 1000))

@app.route('/')
def index():
    return render_template('index.html')


@app.route('/register', methods=['POST'])
def register():
    # make unique user id
    uuid = str(uuid4())
    user = generate_name()
    lat = request.args.get('lat', type=float, default=0)
    lon = request.args.get('lon', type=float, default=0)

    users[uuid] = {"user": user, "lat": lat, "lon": lon, "elo": 1000, "default": False}
    #print(users[uuid])
    return jsonify({"uuid": uuid, "elo": 1000, "user": user})

@app.route('/match', methods=['POST'])
def match():
    uuid = request.args.get('uuid')
    lat = request.args.get('lat', type=float, default = 0)
    lon = request.args.get('lon', type=float, default = 0)
    if (lat <= -90 or lat >= 90 or lon <= -180 or lon >= 180):
        return jsonify({"error": "Invalid coordinates"})
    if uuid not in users:
        return jsonify({"error": "User not found"})
    inc_elo = users[uuid]["elo"] + random.randint(-100, 200)
    only_default = inc_elo >= 3000
    closest_match = None
    closest_uuid = None
    closest_elo_diff = -1
    if users[uuid]["elo"] >= 3000:
        closest_match = users[geopy_uuid]
        closest_uuid = geopy_uuid
    else:
        for user, user_obj in users.items():
            if user == uuid:
                continue
            if only_default and not user_obj["default"]:
                continue
            if closest_match is None or abs(user_obj["elo"] - inc_elo) < closest_elo_diff:
                closest_match = user_obj
                closest_uuid = user
                closest_elo_diff = abs(user_obj["elo"] - inc_elo)

    return jsonify({"uuid": closest_uuid, "user": closest_match["user"], "elo": closest_match["elo"], "distance": distance.distance((lat, lon), (closest_match["lat"], closest_match["lon"])).miles})

@app.route('/battle', methods=['POST'])
def battle():
    uuid = request.args.get('uuid')
    if uuid not in users:
        return jsonify({"error": "User not found"})
    opponent = request.args.get('opponent')
    if opponent not in users:
        return jsonify({"error": "Opponent not found"})
    number = request.args.get('number', type=int)
    if number is None:
        return jsonify({"error": "Number not valid"})
    
    
        
    opponent_number = number + random.randint(0, 10000)
    if users[uuid]["user"] == "geopy":
        opponent_number = number - 2

    if number == opponent_number:
        return jsonify({"result": "draw", elo: users[uuid]["elo"]})

    if number > opponent_number:
        winner = uuid
        loser = opponent
        res = "win"
    else:
        winner = opponent
        loser = uuid
        res = "loss"

    win_elo, lose_elo = elo_calculation(users[winner]["elo"], users[loser]["elo"])

    if (users[winner]["user"] == "geopy"):
        win_elo = 3000
    if (users[loser]["user"] == "geopy"):
        lose_elo = 3000
    
    users[loser]["elo"] = lose_elo 
    users[winner]["elo"] = win_elo
    
    return jsonify({"result": res, "opponent_number": opponent_number, "elo": users[uuid]["elo"]})

@app.route('/elo', methods=['GET'])
def elo(): 
    uuid = request.args.get('uuid')
    return jsonify({"elo": users[uuid]["elo"]})

def elo_calculation(winner_elo, loser_elo):
    k = 32
    expected_winner = 1 / (1 + 10 ** ((loser_elo - winner_elo) / 400))
    expected_loser = 1 / (1 + 10 ** ((winner_elo - loser_elo) / 400))
    winner_elo = winner_elo + k * (1 - expected_winner) + 2
    loser_elo = loser_elo + k * (0 - expected_loser)
    return int(winner_elo), int(loser_elo)

def gen_user():
    uuid = str(uuid4())
    user = generate_name()
    lat = random.uniform(-90, 90)
    lon = random.uniform(-180, 180)
    users[uuid] = {"user": user, "lat": lat, "lon": lon, "elo": random.randint(400, 2970), "default": True}

def init_app():
    global geopy_uuid
    for _ in range(1000):
        gen_user()
    
    geopy_uuid = str(uuid4())
    users[geopy_uuid] = {"user": "geopy", "lat": 39.940418395028665 , "lon": -82.99669527523196, "elo": 3000, "default": True}
    
    
#print(distance.distance((39.94037054169889, -82.99665905631318), (39.940418395028665 , -82.99669527523196)).miles)
init_app()
# print(users) 39.94037054169889, -82.99665905631318
if __name__ == '__main__':
    
    app.run(host='0.0.0.0', port=5000)