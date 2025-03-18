fn main() {
    // Si las bibliotecas estáticas de SDL2 están en una carpeta especial, puedes usar pkg-config.
    // Esto le dice a Rust que busque SDL2 para ver cómo debe enlazarlo.

    println!("cargo:rerun-if-changed=build.rs"); // Esto asegura que se ejecute de nuevo si cambias este archivo

    // Si tienes SDL2 estático, configuramos el enlace
    println!("cargo:rustc-link-lib=static=SDL2");  // Enlaza la biblioteca estática de SDL2

    // Si necesitas más información sobre la ubicación de las bibliotecas, puedes usar pkg-config
    // pkg-config lo configura automáticamente si tienes SDL2 instalado y pkg-config disponible
    println!("cargo:rustc-link-search=native=/ruta/a/las/bibliotecas");
}

