
# **Utilización la biblioteca ZeroMQ para interactuar con la mempool de un nodo de Bitcoin.**

1. **Creación de un contexto ZeroMQ**: El programa comienza creando un nuevo contexto ZeroMQ. Este contexto se utiliza para crear sockets ZeroMQ.

2. **Creación de un socket SUB (suscriptor)**: El programa crea un socket SUB utilizando el contexto ZeroMQ. Si hay un error al crear el socket, el programa imprime el error y lo devuelve.

3. **Conexión al nodo de Bitcoin**: El socket SUB se conecta a un nodo de Bitcoin que se ejecuta en localhost en el puerto 28332. Si hay un error al conectar, el programa imprime el error y lo devuelve. 
En el archivo ***bitcoin.conf*** se ha de incluir el comando: 
zmqpubrawtx=tcp://127.0.0.1:28332

4. **Suscripción a eventos "rawtx"**: El socket SUB se suscribe a los eventos "rawtx", que representan transacciones en la mempool del nodo de Bitcoin. Si hay un error al suscribirse, el programa imprime el error y lo devuelve.

5. **Recepción y procesamiento de mensajes**: El programa entra en un bucle donde recibe y procesa continuamente mensajes. El primer mensaje que recibe es el canal de suscripción, que descarta. El segundo mensaje que recibe es la transacción en formato hexadecimal, que imprime. Si hay un error al recibir cualquiera de los mensajes, el programa imprime el error y continúa con la siguiente iteración del bucle o devuelve el error.

6. **Pausa**: El programa hace una pausa de 3 segundos para evitar el consumo excesivo de recursos.



