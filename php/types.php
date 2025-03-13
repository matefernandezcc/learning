<?php
    //////////////////////////////// Tipos y variables ////////////////////////////////
    
    // Los indentificadores inician con un '$' seguido de una letra o un _
    // No hay que declarar variables, PHP las declara al asignarles un valor

    $int = 1;
    $int2 = -1;
    $float = 3.14;
    $binario = 0b111;
    $octal = 012;
    $hexadecimal = 0x0F;

    $string = 'Testing'; // Los strings van entre comillas simples 'string'
    $boolean = true;
    $boolean = FALSE;

    // Borrar variables
    unset($int);

    // El unico caso donde un string va entre comillas dobles es cuando adentro se llama a otro string
    // otra forma de reemplazar con el string es {$string}
    $mensaje = "$string";
    echo "{$mensaje}";

    /////// Strings ///////
    // Concatenar strings
    $string1 = 'Probando';
    $string2 = 'PHP';
    print("<br>" . "$string1 " . "$string2" . "<br>");

    /////// Constantes ///////
    define("FOO", "Valor constante");
    echo "<br>Constante<br>" .FOO."<br>";



    //////////////////////////////// Arrays ////////////////////////////////
    // Los arrays en PHP son asociativos (hash maps)

    // Sintaxis antigua (Compatible con todas las versiones)
    $array = array('uno'=>1, 'dos'=>2, 'tres'=>3);

    // Sintaxis nueva (PHP 5.4)
    $array2 = ['uno'=>1, 'dos'=>2, 'tres'=>3];
    $array2['cuatro'] = 4; // Agregar elementos

    // Agregar elemento al final del array
    $array2[] = 'cinco';
    // o
    array_push($array2, 'seis');

    //// Sacar elemento
    unset($array[3]); // saca el indice 3

    echo "<br>Arrays<br>"."Array[1] => ".$array2[0];
?>