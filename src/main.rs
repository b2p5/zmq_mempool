//
// Ejemplo de suscripción al nodo de Bitcoin usando ZeroMQ
//  


use std::thread;
use std::time::Duration;
use zmq::Context;

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
    if let Err(e) = subscriber.set_subscribe(b"rawtx") {
        eprintln!("Error al suscribirse: {:?}", e);
        return Err(e.into());
    }

    // Recibir y procesar mensajes
    loop {
        // El primer mensaje contiene el canal de suscripción, por lo que lo descartamos
        let _ = match subscriber.recv_bytes(0) {
            Ok(channel) => channel,
            Err(e) => {
                eprintln!("Error al recibir el canal: {:?}", e);
                continue;
            }
        };
        
        // El segundo mensaje contiene la transacción en formato hexadecimal
        let tx_hex = match subscriber.recv_bytes(0) {
            Ok(tx) => tx,
            Err(e) => {
                // Handle the zmq::Error
                // For now, let's return the error
                return Err(e.into());
            },
        };

        println!("Received transaction: {:?}", tx_hex);

        // Pausar para evitar un consumo excesivo de recursos
        thread::sleep(Duration::from_secs(5));
    
    }   // Fin del loop

}

fn main() {
    // Iniciar el suscriptor en el hilo principal
    match mempool_subscriber() {
        Ok(_) => {
            // La función se ejecutó con éxito, podemos continuar con nuestro código  
        },
        Err(e) => {
            // Hubo un error al ejecutar la función, puedes manejarlo aquí
            eprintln!("Error al ejecutar mempool_subscriber: {:?}", e);
        },
    }
}
