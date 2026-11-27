use std::io;

fn mostrar_tablero(tablero: [[char; 3]; 3]) {
    for col in tablero {
        print!("\n     ");
        for fil in col {
            print!("{fil}   ");
        }
    }
    println!();
}

fn buscar_posicion(mut n: usize) -> (usize, usize) {
    n -= 1;

    let fila = n / 3;
    let col = n % 3;

    (fila, col)
}

fn num_invalido(tablero: [[char; 3]; 3], n: usize) -> bool {
    if n > 9 {
        return true;
    }

    let (fila, col) = buscar_posicion(n);
    tablero[fila][col] == EQ1_CHAR || tablero[fila][col] == EQ2_CHAR
}

fn jugar_lugar(tablero: &mut [[char; 3]; 3], n: usize, eq: char) {
    let (fila, col) = buscar_posicion(n);

    tablero[fila][col] = eq
}

fn ver_ganador(tablero: [[char; 3]; 3]) -> bool {
    // Filas
    for fila in tablero {
        if fila[0] == fila[1] && fila[1] == fila[2] {
            return true;
        }
    }

    // Columnas
    for col in 0..3 {
        if tablero[0][col] == tablero[1][col] && tablero[1][col] == tablero[2][col] {
            return true;
        }
    }

    // Diagonales
    if tablero[0][0] == tablero[1][1] && tablero[1][1] == tablero[2][2] {
        return true;
    }
    if tablero[2][0] == tablero[1][1] && tablero[1][1] == tablero[0][2] {
        return true;
    }

    false
}

const EQ1_CHAR: char = 'X';
const EQ2_CHAR: char = 'O';

fn main() {
    let a: [char; 3] = ['1', '2', '3'];
    let b: [char; 3] = ['4', '5', '6'];
    let c: [char; 3] = ['7', '8', '9'];

    let mut tablero: [[char; 3]; 3] = [a, b, c];

    let mut eq1: bool = true;
    let mut cont: usize = 0;

    loop {
        println!();
        mostrar_tablero(tablero);
        let mut entrada: String = String::new();

        if eq1 {
            println!("Turno de {EQ1_CHAR}");
        } else {
            println!("Turno de {EQ2_CHAR}");
        }

        println!("0 Para salir; indica que posición jugar: ");
        io::stdin().read_line(&mut entrada).expect("Error");

        let n: usize = match entrada.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pone algo como la gente");
                continue;
            }
        };

        if n == 0 {
            break;
        }
        if num_invalido(tablero, n) {
            continue;
        }

        if eq1 {
            jugar_lugar(&mut tablero, n, EQ1_CHAR);

            if ver_ganador(tablero) {
                mostrar_tablero(tablero);
                println!("Gana {EQ1_CHAR}");
                break;
            }

            eq1 = false;
        } else {
            jugar_lugar(&mut tablero, n, EQ2_CHAR);

            if ver_ganador(tablero) {
                mostrar_tablero(tablero);
                println!("Gana {EQ2_CHAR}");
                break;
            }

            eq1 = true;
        }

        cont += 1;
        if cont == 9 {
            mostrar_tablero(tablero);
            println!("Empate, manga de gansos.");
            break;
        }
    }
}
