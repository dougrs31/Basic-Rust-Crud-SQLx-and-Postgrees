#![allow(dead_code)]
use std::error::Error;
use sqlx::FromRow;
use std::io;

#[derive(Debug, FromRow)]
struct Usuario {
    pub nombre: String,
    pub correo: String,
    pub usuarioid: String,
}

async fn create (usuario: &Usuario, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO usuarios (nombre, correo, usuarioid) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&usuario.nombre)
        .bind(&usuario.correo)
        .bind(&usuario.usuarioid)
        .execute(pool)
        .await?;
    
    Ok(())
}

async fn update(usuario: &Usuario, usuarioid: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE usuarios SET nombre = $1, correo = $2 WHERE usuarioid = $3";

    sqlx::query(query)
        .bind(&usuario.nombre)
        .bind(&usuario.correo)
        .bind(&usuario.usuarioid)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Usuario>, Box<dyn Error>> {
    let q = "SELECT nombre, correo, usuarioid FROM usuarios";
    
    let query = sqlx::query_as::<_, Usuario>(q);
    
    let usuarios = query.fetch_all(conn).await?;
    
    for usuario in &usuarios{
        println!("--------------------");
        println!("Nombre: {}", usuario.nombre);
        println!("Correo: {}", usuario.correo);
        println!("Usuario ID: {}", usuario.usuarioid);
    };

    Ok(usuarios)
}

async fn delete(usuarioid: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let q = format!("DELETE FROM usuarios WHERE usuarioid = '{}'", usuarioid);

    let query = sqlx::query(&q).execute(pool).await?;
    println!("Usuario eliminado exitosamente");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://username:password@localhost:5432/usuarios";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let mut opcion = String::new();

    loop {
        println!("Menú de opciones:");
        println!("1. Añadir usuario");
        println!("2. Editar usuario");
        println!("3. Leer usuarios");
        println!("4. Eliminar usuario");
        println!("5. Salir");

        println!("Ingrese su opción: ");
        io::stdin().read_line(&mut opcion).unwrap();

        match opcion.trim() {
        "1" => {
            println!("Ingrese el nombre del usuario:");
            let mut nom = String::new();
            io::stdin().read_line(&mut nom).unwrap();
            
            println!("Ingrese el correo del usuario:");
            let mut email = String::new();
            io::stdin().read_line(&mut email).unwrap();
            
            println!("Ingrese el Id del usuario:");
            let mut identificador = String::new();
            io::stdin().read_line(&mut identificador).unwrap();

            let usuario = Usuario{
                nombre: nom.to_string(),
                correo: email.to_string(),
                usuarioid: identificador.to_string(),
            };
            create(&usuario, &pool).await?;

            println!("Añadiendo usuario...");
        }
        "2" => {
            println!("Ingrese el Id del usuario:");
            let mut identificador = String::new();
            io::stdin().read_line(&mut identificador).unwrap();

            println!("Ingrese el nuevo nombre del usuario:");
            let mut nom = String::new();
            io::stdin().read_line(&mut nom).unwrap();
            
            println!("Ingrese el nuevo correo del usuario:");
            let mut email = String::new();
            io::stdin().read_line(&mut email).unwrap();

            let usuario = Usuario{
                nombre: nom.to_string(),
                correo: email.to_string(),
                usuarioid: identificador.to_string(),
            };
            update(&usuario, &usuario.usuarioid, &pool).await?;
            
            println!("Editando usuario...");
        }
        "3" => {
            let mut espera = String::new();
            println!("Leyendo usuarios...");
            read(&pool).await?;
            println!("--------------------");
            println!("Presione enter para continuar");
            io::stdin().read_line(&mut espera).unwrap();
        }
        "4" => {
            let mut espera = String::new();
            println!("Ingrese el Id del usuario que desea eliminar:");
            let mut identificador = String::new();
            io::stdin().read_line(&mut identificador).unwrap();

            delete(&identificador, &pool).await?;
            println!("Eliminando usuario...");
            println!("--------------------");
            println!("Presione enter para continuar");
            io::stdin().read_line(&mut espera).unwrap();
        }
        "5" => {
            println!("Saliendo del menú...");
            break;
        }
        _ => {
            println!("Opción inválida. Intente nuevamente.");
        }
        }

        opcion.clear();
    }

    Ok(())
}
