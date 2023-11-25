//! This is an example of subscribing to a Bitcoin node using ZeroMQ.
//!
//! The `mempool_subscriber` function creates a ZeroMQ context and a SUB socket, connects to a Bitcoin node,
//! subscribes to mempool-related events, and receives and processes messages.
//!
//! The main function calls `mempool_subscriber` and handles any errors that occur during execution.
//!



use std::thread;
use std::time::Duration;
use zmq::Context;

use rustc_hex::ToHex;

fn mempool_subscriber() -> Result<(), Box<dyn std::error::Error>> {
    
    // Crear un contexto zmq
    let context = Context::new();

    // Crear un socket de tipo SUB
    let subscriber = match context.socket(zmq::SUB) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Error al crear el socket: {:?}", e);
            return Err(e.into());
        }
    };

    // Conectar al nodo Bitcoin  (puerto 28332)
    if let Err(e) = subscriber.connect("tcp://127.0.0.1:28332") {
        eprintln!("Error al conectar al nodo Bitcoin: {:?}", e);
        return Err(e.into());
    }

    // Suscribirse a eventos relacionados con la mempool
    if let Err(e) = subscriber.set_subscribe(b"hashtx") {
    //if let Err(e) = subscriber.set_subscribe(b"rawtx") {
        eprintln!("Error al suscribirse: {:?}", e);
        return Err(e.into());
    }

    // Recibir y procesar mensajes
    loop {
        // El segundo mensaje contiene el contenido del mensaje
        let tx_hex = match subscriber.recv_bytes(0) {
            
            Ok(tx_hex) => {
                // Convert bytes to a hexadecimal string
                let hex_string = tx_hex.to_hex::<String>();
                Some(hex_string)
            },
            Err(e) => {
                println!("Failed to receive bytes: {}", e);
                None
            },
        };
        
        let tx_hex = match tx_hex {
            Some(tx) => tx,
            None => String::from("No transaction received"),
        };
        
        println!("Received transaction: {:?}", tx_hex);
        
        // Pausar para evitar un consumo excesivo de recursos
        thread::sleep(Duration::from_secs(1));
    
    }   // Fin del loop

}

fn main() {
    // Iniciar el suscriptor en el hilo principal
    match mempool_subscriber() {
        Ok(_) => {
            // La función se ejecutó con éxito, podemos continuar con nuestro código 
            println!("Terminado"); 
        },
        Err(e) => {
            // Hubo un error al ejecutar la función, puedes manejarlo aquí
            eprintln!("Error al ejecutar mempool_subscriber: {:?}", e);
        },
    }
}
