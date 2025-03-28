<?php
error_reporting(-1);
ini_set('display_errors', 'On');

echo "<html>";
echo "<head>\n
    <meta charset=\"UTF-8\">\n
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n
    <title>OTP</title>\n
  </head>";
echo "<body>";

$servername = "db-sqli";
$username = "sqli-user";
$password = 'AxU3a9w-azMC7LKzxrVJ^tu5qnM_98Eb';
$dbname = "SqliDB";


// Establish connection exists to mysql
$conn = new mysqli($servername, $username, $password, $dbname);
if ($conn->connect_error)
    die("Connection failed: " . $conn->connect_error);

// SQL query syntax checking for matching user/pass in table login
// $sql = "SELECT * FROM login";
// $people = [];
// $pws = [];
// if ($result = $conn->query($sql)) // Actually starting sql query
// {
//     if ($result->num_rows >= 1) // If matching rows found...
//     {
//         $i = 0;
//         while ($row = mysqli_fetch_array($result)) {
//             // print_r($row);
//             $people[$i] = $row[0];
//             $pws[$i] = $row[1];
//             $i += 1;
//         }
//     }
// } else {
//     die("Unable to load db");
// }

echo "<h1>OTP Matching Service</h1>";
// echo "<h2>Leaderboard</h2>";
// $rankings = [];
// // print_r($pws);
// // print_r($people);
// for ($j = 0; $j < $i; $j++) {
//     for ($k = $j + 1; $k < $i; $k++) {
//         $dist = 0;
//         $pw1 = $pws[$j];
//         $pw2 = $pws[$k];
//         $diff = strlen($pw1) - strlen($pw2);
//         for ($l = 0; $l < $diff; $l++) {
//             $pw2 .= "\0";
//         }
//         for ($l = 0; $l < -$diff; $l++) {
//             $pw1 .= "\0";
//         }
//         $text = $pw1 ^ $pw2;
//         for ($l = 0; $l < strlen($text); $l++) {
//             $dist = $dist + ord($text[$l]);
//         }
//         $rankings[$people[$j] . " x " . $people[$k]] = $dist;
//     }
// }
// // print_r($rankings);
// asort($rankings);
// echo "<ol>";
// $i = 0;
// foreach ($rankings as $key => $val) {
//     echo "<li>" . $key . ": " . $val . "</li>";
//     $i = $i + 1;
//     if ($i == 100) break;
// }
// echo "</ol>";

echo "<h2>Sign up:</h2>";
echo "<form action=\"index.php\" method=\"post\">";
echo "<label for=\"username\">Username:</label>
        <input type=\"text\" name=\"username\" id=\"username\" required>
        <label for=\"password\">Secret:</label>
        <input type=\"password\" name=\"password\" id=\"password\" required>
        <button type=\"submit\">Submit</button>";
echo "</form>";

if (isset($_POST["username"]) && isset($_POST["password"])) {
    $user = $_POST['username'];
    $pass = $_POST['password'];
    $upattern = "/^[A-Za-z0-9\_\{\}]{1,16}$/";
    $ppattern = "/^[A-Za-z0-9\_\{\}]{1,32}$/";
    if (preg_match($upattern, $user) && preg_match($ppattern, $pass)) {
        $sql = "SELECT * FROM login WHERE User='" . $user . "'";
        if ($result = $conn->query($sql)) // Actually starting sql query
        {
            if ($result->num_rows >= 1) // If matching rows found...
            {
                echo "<p>User already exists!</p>";
            } else {
                $sql = "INSERT INTO login (User, Password) VALUES ('" . $user . "', '" . $pass . "')";
                if ($conn->query($sql)) {
                    echo "<p>Successfully registered " . $user . "</p>";
                } else {
                    echo "<p>Database Error " . $conn->error . "</p>";
                }
            }
        } else {
            echo "<p>Database Error " . $conn->error . "</p>";
        }
    } else {
        echo "<p>Invalid user/secret combo. Usernames and secrets can only contain alphanumeric characters, underscore, and curly braces. Usernames must be 1-16 characters long and secrets must be 1-32 characters long.</p>";
    }
}

echo "<h2>Look up a specific pairing:</h2>";
echo "<form action=\"index.php\" method=\"post\">";
echo "<label for=\"username1\">Username 1:</label>
        <input type=\"text\" name=\"username1\" id=\"username1\" required>
        <label for=\"username2\">Username 2:</label>
        <input type=\"text\" name=\"username2\" id=\"username2\" required>
        <button type=\"submit\">Submit</button>";
echo "</form>";

if (isset($_POST["username1"]) && isset($_POST["username2"])) {
    $user1 = $_POST['username1'];
    $user2 = $_POST['username2'];
    $upattern = "/^[A-Za-z0-9\_\{\}]{1,16}$/";
    if (preg_match($upattern, $user1) && preg_match($upattern, $user2)) {
        $sql = "SELECT * FROM login WHERE User='" . $user1 . "'";
        if ($result = $conn->query($sql)) // Actually starting sql query
        {
            if ($result->num_rows >= 1) // If matching rows found...
            {
                $pw1 = "";
                while ($row = mysqli_fetch_array($result)) {
                    // print_r($row);
                    $pw1 = $row[1];
                    break;
                }
                $sql = "SELECT * FROM login WHERE User='" . $user2 . "'";
                if ($result = $conn->query($sql)) // Actually starting sql query
                {
                    if ($result->num_rows >= 1) // If matching rows found...
                    {
                        $dist = 0;
                        $pw2 = "";
                        while ($row = mysqli_fetch_array($result)) {
                            // print_r($row);
                            $pw2 = $row[1];
                            break;
                        }
                        $diff = strlen($pw1) - strlen($pw2);
                        for ($l = 0; $l < $diff; $l++) {
                            $pw2 .= "\0";
                        }
                        for ($l = 0; $l < -$diff; $l++) {
                            $pw1 .= "\0";
                        }
                        $text = $pw1 ^ $pw2;
                        for ($l = 0; $l < strlen($text); $l++) {
                            $dist = $dist + ord($text[$l]);
                        }
                        echo "<p>The pairing for " . $user1 . " and " . $user2 . " is: " . strval($dist) . "</p>";
                    } else {
                        echo "<p>Could not find user 2</p>";
                    }
                }
            } else {
                echo "<p>Could not find user 1</p>";
            }
        } else {
            echo "<p>Database Error " . $conn->error . "</p>";
        }
    } else {
        echo "<p>Invalid username specified!</p>";
    }
}

echo "<h2>Check your top matches</h2>";
echo "<form action=\"index.php\" method=\"post\">";
echo "<label for=\"usernamesearch\">Username:</label>
        <input type=\"text\" name=\"usernamesearch\" id=\"usernamesearch\" required>
        <button type=\"submit\">Submit</button>";
echo "</form>";

if (isset($_POST["usernamesearch"])) {
    $user = $_POST['usernamesearch'];
    $upattern = "/^[A-Za-z0-9\_\{\}]{1,16}$/";
    if (preg_match($upattern, $user)) {
        $sql = "SELECT * FROM login WHERE User='" . $user . "'";
        if ($result = $conn->query($sql)) // Actually starting sql query
        {
            if ($result->num_rows >= 1) // If matching rows found...
            {
                $pw = "";
                while ($row = mysqli_fetch_array($result)) {
                    // print_r($row);
                    $pw = $row[1];
                    break;
                }
            }
            else {
                die("User not found.");
            }
        }
        else {
            die("database error");
        }
        $sql = "SELECT * FROM login";
        $people = [];
        $pws = [];
        if ($result = $conn->query($sql)) // Actually starting sql query
        {
            if ($result->num_rows >= 1) // If matching rows found...
            {
                $i = 0;
                while ($row = mysqli_fetch_array($result)) {
                    // print_r($row);
                    $people[$i] = $row[0];
                    $pws[$i] = $row[1];
                    $i += 1;
                }
            }
        } else {
            die("Unable to load db");
        }
        $rankings = [];
        // print_r($pws);
        // print_r($people);
        for ($j = 0; $j < $i; $j++) {
            // for ($k = $j + 1; $k < $i; $k++) {
                $dist = 0;
                $pw1 = $pws[$j];
                $pw2 = $pw;
                $diff = strlen($pw1) - strlen($pw2);
                for ($l = 0; $l < $diff; $l++) {
                    $pw2 .= "\0";
                }
                for ($l = 0; $l < -$diff; $l++) {
                    $pw1 .= "\0";
                }
                $text = $pw1 ^ $pw2;
                for ($l = 0; $l < strlen($text); $l++) {
                    $dist = $dist + ord($text[$l]);
                }
                $rankings[$people[$j] . " x " . $user] = $dist;
            // }
        }
        // print_r($rankings);
        asort($rankings);
        echo "<ol>";
        $i = 0;
        foreach ($rankings as $key => $val) {
            echo "<li>" . $key . ": " . $val . "</li>";
            $i = $i + 1;
            if ($i == 100) break;
        }
        echo "</ol>";
    }
}

echo "</body>";
echo "</html>";
