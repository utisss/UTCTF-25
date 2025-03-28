let userUUID = null;
let opponentUUID = null;

var lat = 0;
var lon = 0;
// Automatically register user on page load
window.onload = async () => {
    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition(async (position) => {
            lat = position.coords.latitude;
            lon = position.coords.longitude;

            const response = await fetch(`/register?lat=${lat}&lon=${lon}`, { method: 'POST' });
            const data = await response.json();
            userUUID = data.uuid;
            document.getElementById('user-info').innerText = `Welcome, ${data.user}! Elo: ${data.elo}`;
        });
    } else {
        alert("Geolocation is not supported by this browser.");
        const response = await fetch(`/register?lat=${lat}&lon=${lon}`, { method: 'POST' });
        const data = await response.json();
        userUUID = data.uuid;
        document.getElementById('user-info').innerText = `Welcome, ${data.user}! Elo: ${data.elo}`;
    }
};

// Request a match
async function findMatch() {
    const response = await fetch(`/match?uuid=${userUUID}&lat=${lat}&lon=${lon}`, { method: 'POST' });
    const data = await response.json();
    if (data.error) {
        alert(data.error);
        return;
    }
    opponentUUID = data.uuid;
    document.getElementById('match-info').innerText = `Matched with ${data.user} (Elo: ${data.elo}, Distance: ${Math.round(data.distance)} miles)`;
    document.getElementById('match-section').style.display = 'none';
    document.getElementById('battle-section').style.display = 'block';
}

// Submit a battle
async function battle() {
    const number = document.getElementById('number-input').value;
    if (!number) {
        alert("Please enter a number.");
        return;
    }

    const response = await fetch(`/battle?uuid=${userUUID}&opponent=${opponentUUID}&number=${number}`, { method: 'POST' });
    const data = await response.json();
    if (data.error) {
        alert(data.error);
        return;
    }

    document.getElementById('battle-result').innerText = `Result: ${data.result}. Opponent's number: ${data.opponent_number}. Your new Elo: ${data.elo}`;
    document.getElementById('user-info').innerText = `Your updated Elo: ${data.elo}`;
    document.getElementById('battle-section').style.display = 'none';
    document.getElementById('match-section').style.display = 'block';
}