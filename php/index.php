<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link rel="stylesheet" type="text/css" href="./style.css">
        <title>Document</title>
    </head>
    <body>
        <form action="index.php">
            <h1 id="titulo">Curso PHP</h1>
            <a href="./types.php">PHP Data types</a>

            <br><br>
            <label>usuario:</label>
            <input id="usuario-input" type="text" name="user"></input>

            <br><br>
            <label>contraeña:</label>
            <input type="text" name="password"></input>

            <br><br>
            <input id="submit" type="submit" value="Log in"></input>
        </form>
    </body>
</html>
<?php 
    echo"<br><br>GET y POST <br>";
    echo "{$_POST["user"]} <br>";
    echo "{$_POST["password"]} <br>";
?>